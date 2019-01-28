use crate::{U16_MAX, RCOND, FL, sign_extend};

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

    pub fn add(&mut self, instruction: u16) {
        // Destination register, this is where the
        // result of the calculation goes
        let destination = (instruction >> 9) & 0x7;

        // SR1 is always the first operand
        let sr1 = (instruction >> 6) & 0x7;

        // immflag determines what the second operand is
        let immflag = (instruction >> 5) & 0x1;

        // So depending on the encoding, that could be imm5 or SR2
        let operand = {
            if immflag == 1 {
                // imm5 is 5 bits, so we extract those bits and sign_extend them
                sign_extend(instruction & 0x1F, 5)
            } else {
                // SR2
                self.registers[(instruction & 0x7) as usize]
            }
        };

        // Here's where the actual addition happens
        self.registers[destination as usize] = {
            self.registers[sr1 as usize] + operand
        };

        // And then RCOND is updated
        self.update_rcond(destination as usize);
    }
}