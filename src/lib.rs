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
pub enum FL {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2
}  // TODO: Remove bitshifting from this enum

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
    pub fn update_rcond(&mut self, register: u16) {
        let register_val = self.registers[register as usize];
        self.registers[RCOND as usize] = flag(register_val) as u16;
    }
}

// Takes a value and returns a flag indicating if it is negative/zero/positive
pub fn flag(register_val: u16) -> FL {
    if register_val == 0 {
        FL::ZRO
    } else if is_negative_u16(register_val) {
        FL::NEG
    } else {
        FL::POS
    }
}  // TODO: Make this part of the FL enum


// Returns a bool based on whether the number given
// represents a negative number or not
pub const fn is_negative(x: u16, bit_count: u16) -> bool {
    (x >> (bit_count - 1)) & 1 == 1
}  // TODO: Remove bitwise AND operation

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
