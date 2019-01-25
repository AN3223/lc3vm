// Broken off into a module since it's unlikely
// there would be a need to import these,
// but it's public just in case the need arises
pub mod consts {
    // Maximum possible value to store within a u16
    pub const U16_MAX: usize = 1 << 16;

    // Default program counter state
    pub const PC_START: u16 = 0x3000;

    // Default state for the registers
    pub const DEFAULT_REGISTERS: [u16; 10] = [0,0,0,0,0,0,0,0,PC_START,0];

    // Default memory state (all 0's)
    pub const DEFAULT_MEMORY: [u16; U16_MAX] = [0; U16_MAX];
}

use consts::*;

// Represents the whole LC-3
pub struct LC3 {
    pub memory: [u16; U16_MAX],
    pub registers: [u16; 10]
}

impl LC3 {
    fn new() -> LC3 {
        LC3 {
            memory: DEFAULT_MEMORY,
            registers: DEFAULT_REGISTERS
        }
    }
}

// All registers
pub enum Register {
    RR0, RR1, RR2, RR3, RR4,
    RR5, RR6, RR7, RPC, RCOND
}

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
}
