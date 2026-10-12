//! # input
//! This module holds logic for handling user input and entering/exiting raw mode.

use std::io::{self, Read};

unsafe extern "C" {
    fn c_enter_raw_mode();
    fn c_exit_raw_mode();
}

/// # raw_mode()
/// Enter raw mode.
pub fn raw_mode() {
    unsafe { c_enter_raw_mode(); }
}

/// # exit_raw_mode()
/// Exit raw mode.
pub fn exit_raw_mode() {
    unsafe { c_exit_raw_mode(); }
}

/// # Input
/// One keystroke.
pub enum Input {
    Return,
    Character(char),
}

impl Input {
    /// Read a byte of input and return a new Input object.
    pub fn get() -> io::Result<Self> {
        let mut stdin = io::stdin();
        let mut buffer = [0];
        stdin.read(&mut buffer)?;

        let character = buffer[0] as char;

        Ok(match character {
            '\n' => Self::Return,
            _ => Self::Character(character),
        })
    }
}
