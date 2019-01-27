// Maximum possible value to store within a u16
pub const U16_MAX: usize = 1 << 16;

// All registers
pub enum Register {
    RR0, RR1, RR2, RR3, RR4,
    RR5, RR6, RR7, RPC, RCOND
}

use Register::*;

// All opcodes
pub enum OP {
    BR, ADD, LD, ST, JSR, AND, LDR, STR,
    RTI, NOT, LDI, STI, JMP, RES, LEA, TRAP
}

// Condition flags
#[derive(PartialEq, Debug)]
pub enum FL {
    POS = 1,
    ZRO = 2,
    NEG = 4
}

// Makes it easier to check the RCOND
impl From<&LC3> for FL {
    fn from(lc3: &LC3) -> FL {
        match lc3.registers[RCOND as usize] {
            1 => FL::POS,
            2 => FL::ZRO,
            4 => FL::NEG,
            _ => panic!("Attempt to create flag from invalid RCOND")
        }
    }
}

impl From<u16> for FL {
    fn from(register_val: u16) -> FL {
        if register_val == 0 {
            FL::ZRO
        } else if is_negative_u16(register_val) {
            FL::NEG
        } else {
            FL::POS
        }
    }
}

// Memory mapped registers
pub enum MR {
    KBSR = 0xFE00, // Keyboard status
    KBDR = 0xFE02 // Keyboard data
}

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


// Returns a bool based on whether the number given
// represents a negative number or not
pub const fn is_negative(x: u16, bit_count: u16) -> bool {
    x >> (bit_count - 1) == 1
}

// Gives the two's complement for a number
pub const fn complement(x: u16, bit_count: u16) -> u16 {
    x | (0xFFFF << bit_count)
}

// Extends a number out from bit_count to 16 bits while
// retaining its sign
pub fn sign_extend(x: u16, bit_count: u16) -> u16 {
    if is_negative(x, bit_count) {
        complement(x, bit_count)
    } else {
        x
    }
}

// Short for is_negative(x, 16)
pub const fn is_negative_u16(x: u16) -> bool {
    is_negative(x, 16)
}

// Short for sign_extend(x, 16)
pub fn sign_extend_u16(x: u16) -> u16 {
    sign_extend(x, 16)
}
