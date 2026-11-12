//! # render
//! This module holds the `Render` struct, which is how rendered widgets are stored.

/// # Render
/// A rendered widget, split into lines.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Render {
    lines: Vec<String>,
    width: u32,
    pub context: RenderContext,
}

impl Render {
    /// Construct a new `Render` with a position context and a `String`.
    pub fn new(value: String, context: RenderContext) -> Self {
        let mut lines = Vec::new();
        
        /*
        let mut width = 0;
        for line in value.split('\n') {
            let line = line.to_string();

            let len = line_length(line.clone());
            if len > width {
                width = len;
            }
            if let Some(max_width) = context.max_width
                && width as u32 > max_width
            {
                width = max_width as usize;
            }

            lines.push(line);
        }
         */

        for line in value.split('\n') {
            lines.push(line.to_string());
        }

        Self {
            lines,
            width: block_width(&value) as u32,
            context,
        }
    }

    /// Get the width of a `Render`.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of a `Render`.
    pub fn height(&self) -> u32 {
        self.lines.len() as u32
    }
}

impl From<Render> for String {
    /// Convert the `Render` into a string with ANSI codes. Final step of the render pipeline.
    fn from(value: Render) -> Self {
        let mut buffer = String::new();
        let x = value.context.x;
        let mut y = value.context.y;

        for mut line in value.lines {
            buffer.push_str(format!("\x1b[{y};{x}H").as_str());

            if let Some(max_width) = value.context.max_width {
                if line_length(line.clone()) > (max_width - 1) as usize {
                    while line_length(line.clone()) > (max_width - 3) as usize {
                        line.pop();
                    }
                    line.push_str("...\x1b[0m");
                }
            }

            buffer.push_str(line.as_str());

            y += 1;
        }

        buffer
    }
}

/// # RenderContext
/// Holds size and location information that a `Render` uses to draw into a `String`.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct RenderContext {
    /// The x coordinate of where a widget will be drawn.
    pub x: u32,

    /// The y coordinate of where a widget will be drawn.
    pub y: u32,

    /// The width of the terminal an application is running in.
    pub terminal_width: u32,

    /// The maximum width this widget is allowed to have.
    pub max_width: Option<u32>,

    /// The horizontal padding to apply between widgets.
    pub row_pad: u32,
}

impl Default for RenderContext {
    fn default() -> Self {
        let width = unsafe { crate::size::terminal_width() } as u32;
        Self {
            x: 1,
            y: 1,
            terminal_width: width,
            max_width: Some(width),
            row_pad: 2,
        }
    }
}

pub fn line_length(line: String) -> usize {
    let mut len = 0;

    for segment in line.split("\x1b[") {
        if segment.len() < 3 { continue; }

        let segment_len = segment.len();
        if segment_len > len {
            len = segment_len;
        }
    }

    len
}

/*
fn split_lines(line: &String) -> Vec<String> {
    let mut buffer = String::new();
    let mut lines = Vec::new();

    for character in line.chars() {
        buffer.push(character);
        if buffer.ends_with("H ") || buffer.ends_with("H\u{e5ff} ") || buffer.ends_with('\n') {
            lines.push(buffer.clone());
            buffer.clear();
        }
    }

    lines.push(buffer);

    lines
}
*/

fn block_width(line: &String) -> usize {
    let mut width = 0;

    let mut good = false;
    for line in line.split("LINEBREAK") {
        for character in line.chars() {
            if character != ' ' && character != 'H' && character != '\x1b' && character != '[' && character != ';' && !character.is_numeric() {
                good = true;
                break;
            }
        }

        if good {
            let length = line_length(line.to_string());
            if length > width {
                width = length;
            }
        }
        good = false;
    }

    width
}
