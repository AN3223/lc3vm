/* This file is focused on representing the memory of the LC-3 */

use crate::{U16_MAX, MR};

extern crate termios;
use std::io::{stdin, Read};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::time::Duration;

const STDIN_FD: i32 = 0;

pub struct Memory {
    pub contents: [u16; U16_MAX]
}

impl Memory {
    pub const fn new() -> Memory {
        Memory { contents: [0; U16_MAX] }
    }

    // Getter for accessing the memory. The memory only
    // needs to be accessed through this getter if the
    // address could be KBSR.

    // Otherwise, directly indexing into the memory is
    // perfectly acceptable.
    pub fn get(&mut self, address: usize) -> u16 {
        *self.getref(address)
    }

    pub fn getref(&mut self, address: usize) -> &mut u16 {
        if address == MR::KBSR as usize {
            let key = check_key();
            if key != 0 {
                self.contents[MR::KBSR as usize] = 1 << 15;
                self.contents[MR::KBDR as usize] = key;
            } else {
                self.contents[MR::KBSR as usize] = 0;
            }
        }
        &mut self.contents[address]
    }

    pub fn set(&mut self, address: usize, to: u16) {
        *self.getref(address) = to;
    }
}

pub fn get_key(tx: &mut Sender<u16>) {
    let mut buffer = [0];
    stdin().read_exact(&mut buffer).unwrap();
    // Reads a single character to the buffer from STDIN

    // Puts the character in the Sender so check_key() can use it
    tx.send(buffer[0] as u16).ok();
    // Ignores any send errors, doesn't matter
    // if the other end gets anything
}

pub fn check_key() -> u16 {
    let termios = Termios::from_fd(STDIN_FD).unwrap();
    // STDIN's original state
    
    let mut new_termios = termios.clone();
    // STDIN's new state that will be modified

    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FD, TCSANOW, &new_termios).unwrap();
    // Sets up the terminal to be able to
    // give individual characters w/o linebreaks

    let (mut tx, rx) = channel();
    thread::spawn(move || {
        get_key(&mut tx)
    }); // TODO: Implement this as a daemon?
    // Spawn get_key() in a new thread

    let timeout = Duration::from_millis(50);
    let key = rx.recv_timeout(timeout);
    // Considers get_key() failed if the Receiver is empty when the timeout is over
    // so the program will not hang forever waiting input
    
    tcsetattr(STDIN_FD, TCSANOW, &termios).unwrap();
    // Return STDIN to its original state,
    // regardless of get_key()'s result

    match key {
        Ok(character) => character,
        Err(_) => 0
    }
    // Returns 0 if get_key() failed.
    // This is intended because 0 represents NULL in ASCII.
}