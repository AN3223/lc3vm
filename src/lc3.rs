use crate::{U16_MAX, RCOND, FL, sign_extend, check_key, MR, Register};
use Register::*;

// Represents the whole LC-3
pub struct LC3 {
    pub memory: [u16; U16_MAX],
    pub register: [u16; 10]
}

impl LC3 {
    pub const fn new() -> LC3 {
        LC3 {
            memory: [0; U16_MAX],
            register: [0,0,0,0,0,0,0,0,0x3000,0]
        }
    }

    // Updates RCOND based on the value of a given register
    pub fn update_rcond(&mut self, register: usize) {
        let register_val = self.register[register];
        self.register[RCOND as usize] = FL::from(register_val) as u16;
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
        let destination_register = (instruction >> 9 & 0x7) as usize;
        let sr1 = (instruction >> 6 & 0x7) as usize; // First operand

        let immflag = instruction >> 5 & 0x1; // Determines what the second operand is
        let operand = {
            if immflag == 1 {
                sign_extend(instruction & 0x1F, 5)
            } else {
                let sr2 = (instruction & 0x7) as usize;
                self.register[sr2]
            }
        };

        // Here's where the actual addition happens
        self.register[destination_register] = {
            self.register[sr1].wrapping_add(operand)
        };

        self.update_rcond(destination_register);
    }

    // Load indirect
    pub fn ldi(&mut self, instruction: u16) {
        let pcoffset = sign_extend(instruction & 0x1ff, 9);
        let destination_register = instruction >> 9 & 0x7;

        let location = self.get_memory(
            (self.register[RPC as usize].wrapping_add(pcoffset)) as usize
        );

        self.register[destination_register as usize] = self.get_memory(location as usize);

        self.update_rcond(destination_register as usize);
    }

    pub fn and(&mut self, instruction: u16) {
        let destination_register = (instruction >> 9 & 0x7) as usize;
        let sr1 = (instruction >> 6 & 0x7) as usize; // First operand

        let immflag = instruction >> 5 & 0x1; // Determines second operand
        let operand = {
            if immflag == 1 {
                sign_extend(instruction & 0x1f, 5)
            } else {
                let sr2 = (instruction & 0x7) as usize;
                self.register[sr2]
            }
        };

        self.register[destination_register] = {
            self.register[sr1] & operand
        };

        self.update_rcond(destination_register);
    }

    pub fn not(&mut self, instruction: u16) {
        let destination_register = (instruction >> 9 & 0x7) as usize;
        let sr1 = (instruction >> 6 & 0x7) as usize;

        self.register[destination_register] = !self.register[sr1];

        self.update_rcond(destination_register);
    }

    pub fn br(&mut self, instruction: u16) {
        let pcoffset = sign_extend(instruction & 0x1ff, 9);
        let cond_flag = instruction >> 9 & 7;

        if cond_flag & self.register[RCOND as usize] != 0 {
            // Increment RPC
            self.register[RPC as usize] = {
                self.register[RPC as usize].wrapping_add(pcoffset)
            };
        }
    }

    pub fn jmp(&mut self, instruction: u16) {
        let base_r = instruction >> 6 & 0x7;
        self.register[RPC as usize] = base_r;
    }

    pub fn jsr(&mut self, instruction: u16) {
        let flag = instruction >> 11 & 0x1;
        
        self.register[RR7 as usize] = self.register[RPC as usize];

        if flag == 1 {
            let pcoffset = sign_extend(instruction & 0x7ff, 11);

            // Increment RPC
            self.register[RPC as usize] = {
                self.register[RPC as usize].wrapping_add(pcoffset)
            }
        } else {
            let base_r = instruction >> 6 & 0x7;
            self.register[RPC as usize] = base_r;
        }
    }
}
