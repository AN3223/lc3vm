/* https://justinmeiners.github.io/lc3-vm/supplies/lc3-isa.pdf */

use crate::{FL, sign_extend, R};
use crate::memory::Memory;

// Represents the whole LC-3
pub struct LC3 {
    pub memory: Memory,
    pub register: [u16; 10]
}

impl LC3 {
    pub const fn new() -> LC3 {
        LC3 {
            memory: Memory::new(),
            register: [0,0,0,0,0,0,0,0,0x3000,0]
        }
    }

    // Updates COND based on the value of a given register
    pub fn update_cond(&mut self, register: usize) {
        let register_val = self.register[register];
        self.register[R::COND as usize] = FL::from(register_val) as u16;
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

        self.update_cond(destination_register);
    }

    // Load indirect
    pub fn ldi(&mut self, instruction: u16) {
        let pcoffset = sign_extend(instruction & 0x1ff, 9);
        let destination_register = instruction >> 9 & 0x7;

        let location = self.memory.get(
            (self.register[R::PC as usize].wrapping_add(pcoffset)) as usize
        );

        self.register[destination_register as usize] = self.memory.get(location as usize);

        self.update_cond(destination_register as usize);
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

        self.update_cond(destination_register);
    }

    pub fn not(&mut self, instruction: u16) {
        let destination_register = (instruction >> 9 & 0x7) as usize;
        let sr1 = (instruction >> 6 & 0x7) as usize;

        self.register[destination_register] = !self.register[sr1];

        self.update_cond(destination_register);
    }

    pub fn br(&mut self, instruction: u16) {
        let pcoffset = sign_extend(instruction & 0x1ff, 9);
        let cond_flag = instruction >> 9 & 7;

        if cond_flag & self.register[R::COND as usize] != 0 {
            // Increment PC
            self.register[R::PC as usize] = {
                self.register[R::PC as usize].wrapping_add(pcoffset)
            };
        }
    }

    pub fn jmp(&mut self, instruction: u16) {
        let base_r = (instruction >> 6 & 0x7) as usize;
        self.register[R::PC as usize] = self.register[base_r];
    }

    pub fn jsr(&mut self, instruction: u16) {
        let flag = instruction >> 11 & 0x1;
        
        self.register[R::R7 as usize] = self.register[R::PC as usize];

        if flag == 1 {
            let pcoffset = sign_extend(instruction & 0x7ff, 11);

            // Increment PC
            self.register[R::PC as usize] = {
                self.register[R::PC as usize].wrapping_add(pcoffset)
            }
        } else {
            let base_r = (instruction >> 6 & 0x7) as usize;
            self.register[R::PC as usize] = self.register[base_r];
        }
    }

    pub fn ld(&mut self, instruction: u16) {
        let pcoffset = sign_extend(instruction & 0xff, 9);
        let destination_register = (instruction >> 9 & 0x7) as usize;

        let pc_incremented = self.register[R::PC as usize] + pcoffset;
        self.register[destination_register] = self.memory.get(pc_incremented as usize);

        self.update_cond(destination_register);
    }

    pub fn ldr(&mut self, instruction: u16) {
        let destination_register = (instruction >> 9 & 0x7) as usize;
        let base_r = (instruction >> 6 & 0x7) as usize;
        let offset = sign_extend(instruction & 0x3f, 6);

        self.register[destination_register] = {
            self.memory.get(
                (self.register[base_r] + offset) as usize
            )
        };

        self.update_cond(destination_register);
    }

    pub fn lea(&mut self, instruction: u16) {
        let destination_register = (instruction >> 9 & 0x7) as usize;
        let pcoffset = sign_extend(instruction & 0x1ff, 9);

        let pc_incremented = pcoffset + self.register[R::PC as usize];
        self.register[destination_register] = pc_incremented;

        self.update_cond(destination_register);
    }

    pub fn st(&mut self, instruction: u16) {
        let sr = (instruction >> 9 & 0x7) as usize;
        let pcoffset = sign_extend(instruction & 0x1f, 9);
        let pc_incremented = self.register[R::PC as usize] + pcoffset;

        self.memory.set(pc_incremented as usize, self.register[sr]);
    }

    pub fn sti(&mut self, instruction: u16) {
        let sr = (instruction >> 9 & 0x7) as usize;
        let pcoffset = sign_extend(instruction & 0x1ff, 9);
        let pc_incremented = pcoffset + self.register[R::PC as usize];

        let location = self.memory.get(pc_incremented as usize);
        self.memory.set(location as usize, self.register[sr]);
    }
}
