/* This file is focused on representing the memory of the LC-3 */

use crate::{U16_MAX, MR};

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

// TODO: This just needs to be rewritten entirely. Returns a 0 as a stand in.
pub fn check_key() -> u16 {
    0
}