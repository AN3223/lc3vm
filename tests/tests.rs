use lc3vm::*;

const NEGATIVE_NUM: u16 = 0b1111111111111111;
const POSITIVE_NUM: u16 = 0b0111111111111111;

#[test]
fn sti() {
    let mut lc3 = LC3::new();
    lc3.memory.set(0x3001, 500);
    lc3.register[0] = 1000;

    let instruction = 0b1011_000_000000001;
    lc3.sti(instruction);

    assert_eq!(lc3.memory.contents[500], 1000);
}

#[test]
fn st() {
    let mut lc3 = LC3::new();
    lc3.register[0] = 500;
    
    let instruction = 0b0011_000_000000001;
    lc3.st(instruction);

    assert_eq!(lc3.memory.contents[0x3001], 500);
}

#[test]
fn lea() {
    let mut lc3 = LC3::new();
    let instruction = 0b1110_000_000000001;
    
    lc3.lea(instruction);
    assert_eq!(lc3.register[0], 0x3001);
}

#[test]
fn ldr() {
    let mut lc3 = LC3::new();
    lc3.register[1] = 50;
    lc3.memory.set(51, 500);
    
    let instruction = 0b0110_000_001_000001;
    lc3.ldr(instruction);
    assert_eq!(lc3.register[0], 500);
}

#[test]
fn ld() {
    let mut lc3 = LC3::new();
    lc3.memory.set(0x3001, 500);
    
    let instruction = 0b0010_000_00000001;
    lc3.ld(instruction);
    
    assert_eq!(lc3.register[0], 500);
}

#[test]
fn jsr() {
    let mut lc3 = LC3::new();
    let instruction = 0b0100_1_00000000001;
    lc3.jsr(instruction);

    assert_eq!(lc3.register[R::R7 as usize], 0x3000);
    // Test that RR7 was correctly set
    assert_eq!(lc3.register[R::PC as usize], 0x3001);
    // Test that PC was correctly incremented

    let instruction = 0b0100_0_00_111_000000;
    lc3.jsr(instruction);

    assert_eq!(lc3.register[R::R7 as usize], 0x3001);
    // Test that RR7 was correctly set to the result of the last operation
    assert_eq!(lc3.register[R::PC as usize], 0x3001);
    // Test that PC was correctly set to the contents of BaseR
}

#[test]
fn jmp() {
    let mut lc3 = LC3::new();
    lc3.register[R::R4 as usize] = 500;

    let instruction = 0b1100_000_100_000000;
    // Set program counter to the contents of RR4

    lc3.jmp(instruction);
    assert_eq!(
        lc3.register[R::PC as usize],
        500
    )
}

#[test]
fn br() {
    let mut lc3 = LC3::new();
    lc3.add(0b0001_000_000_1_00001);
    // Just adding 1 to the 0th register to make COND POS

    let instruction = 0b0000_001_000000001;
    lc3.br(instruction);
    // Should increment PC to 0x3001 since COND is POS

    assert_eq!(lc3.register[R::PC as usize], 0x3001);

    lc3.add(0b0001_000_000_1_11111);

    let instruction = 0b0000_010_000000001;
    lc3.br(instruction);

    assert_eq!(lc3.register[R::PC as usize], 0x3002);
    // Should increment PC again since COND is ZRO

    lc3.add(0b0001_000_000_1_11111);

    let instruction = 0b0000_100_000000001;
    lc3.br(instruction);

    assert_eq!(lc3.register[R::PC as usize], 0x3003);
    // Should increment PC again since COND is NEG
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
    lc3.memory.set(0x3000, 500);
    lc3.memory.set(500, 123);

    let instruction: u16 = 0b1010_000_000000000;
    // Tells the machine to look at memory address 0x3000 (the default PC value),
    // and then to look at the memory address stored within memory address 0x3000,
    // and then store that value within the destination register (zero).
    // Hence load "indirect"

    lc3.ldi(instruction);
    assert_eq!(lc3.register[0], 123);

    lc3.memory.set(0x3001, 501);
    lc3.memory.set(501, 1234);
    let instruction: u16 = 0b1010_000_000000001;

    lc3.ldi(instruction);
    assert_eq!(lc3.register[0], 1234);
}

#[test]
fn add() {
    let mut lc3 = LC3::new();

    // 15 + 0
    let instruction: u16 = 0b0001_000_000_1_01111;
    lc3.add(instruction);
    assert_eq!(lc3.register[0], 15);
    assert_eq!(FL::from(&lc3), FL::POS);

    // 15 + 15
    let instruction: u16 = 0b0001_001_000_0_00_000;
    lc3.add(instruction);
    assert_eq!(lc3.register[1], 30);
    assert_eq!(FL::from(&lc3), FL::POS);

    // 30 - 15
    let instruction: u16 = 0b0001_001_001_1_10001;
    lc3.add(instruction);
    assert_eq!(lc3.register[1], 15);
    assert_eq!(FL::from(&lc3), FL::POS);

    // 15 - 15
    lc3.add(instruction);
    assert_eq!(lc3.register[1], 0);
    assert_eq!(FL::from(&lc3), FL::ZRO);

    // 0 - 15
    lc3.add(instruction);
    assert_eq!(lc3.register[1], 65521);
    assert_eq!(FL::from(&lc3), FL::NEG);
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
    lc3.update_cond(0);
    assert_eq!(FL::from(&lc3), FL::ZRO);

    // Test NEG
    lc3.register[0] = NEGATIVE_NUM;
    lc3.update_cond(0);
    assert_eq!(FL::from(&lc3), FL::NEG);

    // Test POS
    lc3.register[0] = POSITIVE_NUM;
    lc3.update_cond(0);
    assert_eq!(FL::from(&lc3), FL::POS);
}

// Ensures that U16_MAX really is the
// maximum possible u16 value
#[test]
fn correct_u16_max() {
    assert_eq!(U16_MAX, 65536);      
}
