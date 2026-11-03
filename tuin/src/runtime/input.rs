//! # input
//! This module holds logic for handling user input and entering/exiting raw mode.

use crate::log;

use std::io::{self, Read};

unsafe extern "C" {
    fn c_enter_raw_mode();
    fn c_exit_raw_mode();
    fn c_set_stdin_nonblocking();
}

/// # raw_mode()
/// Enter raw mode.
pub fn raw_mode() {
    unsafe {
        c_enter_raw_mode();
    }
}

/// # exit_raw_mode()
/// Exit raw mode.
pub fn exit_raw_mode() {
    unsafe {
        c_exit_raw_mode();
    }
}

/// # set_stdin_nonblocking()
/// Make file descriptor 0 not block for reads
pub fn set_stdin_nonblocking() {
    unsafe {
        c_set_stdin_nonblocking();
    }
}

/// # Input
/// One keystroke.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Input {
    Return,
    Tab,
    Up,
    Down,
    Left,
    Right,
    Backspace,
    Escape,
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
            '\t' => Self::Tab,
            '\x1b' => Self::parse_escape()?,
            '\u{7f}' => Self::Backspace,
            _ => Self::Character(character),
        })
    }

    fn parse_escape() -> io::Result<Self> {
        let mut stdin = io::stdin();

        let mut buffer = [0, 0];
        if let Err(e) = stdin.read(&mut buffer) {
            log(format!("Couldn't read next character because {e}. Returning Input::Escape"));
            return Ok(Self::Escape);
        }

        let sequence = match String::from_utf8(buffer.to_vec()) {
            Ok(seq) => seq,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        };

        Ok(match sequence.as_str() {
            "[A" => Self::Up,
            "[B" => Self::Down,
            "[C" => Self::Right,
            "[D" => Self::Left,
            _ => Self::Character(' '), // I should probably fix this later
        })
    }
}

impl From<Input> for char {
    fn from(value: Input) -> Self {
        match value {
            Input::Return => '\n',
            Input::Character(c) => c,
            Input::Tab => '\t',
            _ => ' ',
        }
    }
}
