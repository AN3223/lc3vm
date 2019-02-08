use crate::{U16_MAX, RCOND, FL, sign_extend, check_key, MR, Register};
use Register::*;

// Represents the whole LC-3
pub struct LC3 {
    pub memory: [u16; U16_MAX],
    pub registers: [u16; 10]
}

impl LC3 {
    pub const fn new() -> LC3 {
        LC3 {
            memory: [0; U16_MAX],
            registers: [0,0,0,0,0,0,0,0,0x3000,0]
        }
    }

    // Updates RCOND based on the value of a given register
    pub fn update_rcond(&mut self, register: usize) {
        let register_val = self.registers[register];
        self.registers[RCOND as usize] = FL::from(register_val) as u16;
    }

    // Getter for accessing the memory. The memory only
    // needs to be accessed through this getter if the
    // address could be MR::KBSR.

    // Otherwise, directly indexing into the memory is
    // perfectly acceptable.
    pub fn get_memory(&mut self, address: usize) -> u16 {
        if address == MR::KBSR as usize {
            let key = check_key();
            if key != 0 {
                self.memory[MR::KBSR as usize] = 1 << 15;
                self.memory[MR::KBDR as usize] = key;
            } else {
                self.memory[MR::KBSR as usize] = 0;
            }
        }
        self.memory[address]
    }

    pub fn add(&mut self, instruction: u16) {
        let destination_register = instruction >> 9 & 0x7;
        let sr1 = instruction >> 6 & 0x7; // First operand

        let immflag = instruction >> 5 & 0x1; // Determines what the second operand is
        let operand = {
            if immflag == 1 {
                sign_extend(instruction & 0x1F, 5)
            } else {
                self.registers[(instruction & 0x7) as usize] // SR2
            }
        };

        // Here's where the actual addition happens
        self.registers[destination_register as usize] = {
            self.registers[sr1 as usize] + operand
        };

        self.update_rcond(destination_register as usize);
    }

    // Load indirect
    pub fn ldi(&mut self, instruction: u16) {
        let pcoffset = sign_extend(instruction & 0x1ff, 9);
        let destination_register = instruction >> 9 & 0x7;

        let location = self.get_memory(
            (self.registers[RPC as usize] + pcoffset) as usize
        );

        self.registers[destination_register as usize] = self.get_memory(location as usize);

        self.update_rcond(destination_register as usize);
    }
}