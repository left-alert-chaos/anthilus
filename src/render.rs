//! # render
//! Provide rendering logic to the rest of Anthilus.

use crate::size::terminal_width;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RenderContext {
    pub x: u32,
    pub y: u32,
    pub width: u32,
}

impl RenderContext {
    pub fn new() -> Self {
        unsafe {
            Self {
                x: 1,
                y: 1,
                width: terminal_width() as u32,
            }
        }
    }
}

pub struct Render {
    pub text: String,
    pub width: usize,
}

impl Render {
    pub fn new(text: String) -> Self {
        let mut greatest_length = 0;
        for line in text.split('\n') {
            let length = line_length(line.to_string());
            if length > greatest_length {
                greatest_length = length;
            }
        }

        Self {
            text,
            width: greatest_length,
        }
    }
}

pub fn line_length(line: String) -> usize {
    let mut len = 0;

    for segment in line.split("\x1b[") {
        if segment.len() < 3 {
            continue;
        }

        let segment_len = segment.len();
        if segment_len > len {
            len = segment_len;
        }
    }

    len
}
