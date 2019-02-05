mod lc3;
pub use lc3::LC3;

extern crate termios;
use std::io;
use std::io::{Read};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::time::Duration;

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

const STDIN: i32 = 0;

pub fn get_key(tx: &mut Sender<u16>, termios: &mut Termios) {
    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN, TCSANOW, termios).unwrap();
    // Sets up the terminal to be able to
    // give individual characters w/o linebreaks

    let mut buffer = [0];
    io::stdin().read_exact(&mut buffer).unwrap();
    // Reads a single character to the buffer from STDIN

    // Puts the character in the Sender so check_key() can use it
    tx.send(buffer[0] as u16).ok();
    // Ignores any send errors, doesn't matter
    // if the other end gets anything
}

pub fn check_key() -> u16 {
    let termios = Termios::from_fd(STDIN).unwrap();
    // STDIN's original state

    let (mut tx, rx) = channel();
    thread::spawn(move || {
        get_key(&mut tx, &mut termios.clone())
    });
    // Spawn get_key() in a new thread

    let timeout = Duration::from_millis(50);
    let key = rx.recv_timeout(timeout);
    // Considers get_key() failed if the Receiver is empty when the timeout is over
    // so the program will not hang forever waiting input
    
    tcsetattr(STDIN, TCSANOW, &termios).unwrap();
    // Return STDIN to its original state,
    // regardless of get_key()'s result

    match key {
        Ok(character) => character,
        Err(_) => 0
    }
    // Returns 0 if get_key() failed.
    // This is intended because 0 represents NULL in ASCII.
}