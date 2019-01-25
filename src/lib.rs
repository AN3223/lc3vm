// Maximum possible value to store within a u16
pub const U16_MAX: usize = 1 << 16;

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
}
