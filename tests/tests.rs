use lc3vm::*;
use Register::*;

const NEGATIVE_NUM: u16 = 0b1111111111111111;
const POSITIVE_NUM: u16 = 0b0111111111111111;

#[test]
fn br() {
    let mut lc3 = LC3::new();
    lc3.add(0b0001_000_000_1_00001);
    // Just adding 1 to the 0th register to make RCOND POS

    let instruction = 0b0000_001_000000001;
    lc3.br(instruction);
    // Should increment RPC to 0x3001 since RCOND is POS

    assert_eq!(lc3.register[RPC as usize], 0x3001);
}

#[test]
fn not() {
    let mut lc3 = LC3::new();
    lc3.register[0] = 0b1111;
    
    let instruction = 0b1001_000_000_1_11111;
    lc3.not(instruction);

    assert_eq!(FL::from(&lc3), FL::NEG);
    assert_eq!(lc3.register[0], 0xffff - 15);
}

#[test]
fn and() {
    let mut lc3 = LC3::new();
    lc3.register[0] = 0b11111;

    let instruction = 0b0101_000_000_1_01010;
    lc3.and(instruction);

    assert_eq!(lc3.register[0], 0b01010);

    lc3.and(instruction);
    assert_eq!(lc3.register[0], 0b01010);
    // Running the same instruction should produce the same results
}

#[test]
fn ldi() {
    let mut lc3 = LC3::new();
    lc3.memory[0x3000] = 500;
    lc3.memory[500] = 123;

    let instruction: u16 = 0b1010_000_000000000;
    // Tells the machine to look at memory address 0x3000 (the default RPC value),
    // and then to look at the memory address stored within memory address 0x3000,
    // and then store that value within the destination register (zero).
    // Hence load "indirect"

    lc3.ldi(instruction);
    assert_eq!(lc3.register[0], 123);

    lc3.memory[0x3001] = 501;
    lc3.memory[501] = 1234;
    let instruction: u16 = 0b1010_000_000000001;

    lc3.ldi(instruction);
    assert_eq!(lc3.register[0], 1234);
}

#[test]
fn add() {
    let mut lc3 = LC3::new();

    // 15 + 0
    let instruction: u16 = 0b0001_000_001_1_01111;
    lc3.add(instruction);
    assert_eq!(lc3.register[0], 15);
    assert_eq!(FL::from(&lc3), FL::POS);

    // 15 + 15
    let instruction: u16 = 0b0001_001_000_0_00_000;
    lc3.add(instruction);
    assert_eq!(lc3.register[1], 30);
    assert_eq!(FL::from(&lc3), FL::POS);
}


// Test flag (FL) functionality
#[test]
fn flags() {
    // Test ZRO
    assert_eq!(
        FL::from(0) as u16,
        FL::ZRO as u16
    );

    // Test NEG
    assert_eq!(
        FL::from(NEGATIVE_NUM) as u16,
        FL::NEG as u16
    );

    // Test POS
    assert_eq!(
        FL::from(POSITIVE_NUM) as u16,
        FL::POS as u16
    );
}

// Test LC3's flag-setting functionality
#[test]
fn flag_setting() {
    let mut lc3 = LC3::new();
    
    // Test ZRO
    lc3.update_rcond(0);
    assert_eq!(FL::from(&lc3), FL::ZRO);

    // Test NEG
    lc3.register[0] = NEGATIVE_NUM;
    lc3.update_rcond(0);
    assert_eq!(FL::from(&lc3), FL::NEG);

    // Test POS
    lc3.register[0] = POSITIVE_NUM;
    lc3.update_rcond(0);
    assert_eq!(FL::from(&lc3), FL::POS);
}

// Ensures that U16_MAX really is the
// maximum possible u16 value
#[test]
fn correct_u16_max() {
    assert_eq!(U16_MAX, 65536);      
}
