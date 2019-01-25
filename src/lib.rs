pub const U16_MAX: usize = 1 << 16;

pub struct LC3 {
    pub memory: [u16; U16_MAX],
    pub registers: [u16; 10]
}

pub enum Register {
    RR0, RR1, RR2, RR3, RR4,
    RR5, RR6, RR7, RPC, RCOND
}

pub enum OP {
    BR, ADD, LD, ST, JSR, AND, LDR, STR,
    RTI, NOT, LDI, STI, JMP, RES, LEA, TRAP
}

#[cfg(test)]
pub mod tests {
    use crate::*;
    use Register::*;

    // Tests that the Register enums
    // correctly cast into u16 values
    #[test]
    fn registers_as_u16() {
        assert_eq!(RR0 as u16,   0);
        assert_eq!(RR1 as u16,   1);
        assert_eq!(RR2 as u16,   2);
        assert_eq!(RR3 as u16,   3);
        assert_eq!(RR4 as u16,   4);
        assert_eq!(RR5 as u16,   5);
        assert_eq!(RR6 as u16,   6);
        assert_eq!(RR7 as u16,   7);
        assert_eq!(RPC as u16,   8);
        assert_eq!(RCOND as u16, 9);
    }

    // Tests that the opcodes correctly
    // cast into u16 values
    #[test]
    fn ops_as_u16() {
        assert_eq!(OP::BR as u16,   0);
        assert_eq!(OP::ADD as u16,  1);
        assert_eq!(OP::LD as u16,   2);
        assert_eq!(OP::ST as u16,   3);
        assert_eq!(OP::JSR as u16,  4);
        assert_eq!(OP::AND as u16,  5);
        assert_eq!(OP::LDR as u16,  6);
        assert_eq!(OP::STR as u16,  7);
        assert_eq!(OP::RTI as u16,  8);
        assert_eq!(OP::NOT as u16,  9);
        assert_eq!(OP::LDI as u16,  10);
        assert_eq!(OP::STI as u16,  11);
        assert_eq!(OP::JMP as u16,  12);
        assert_eq!(OP::RES as u16,  13);
        assert_eq!(OP::LEA as u16,  14);
        assert_eq!(OP::TRAP as u16, 15);
    }

    // Ensures that U16_MAX is the maximum
    // possible u16 value
    #[test]
    fn correct_u16_max() {
        assert_eq!(U16_MAX, 65536);
    }
}