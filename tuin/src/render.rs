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
        let mut width = 0;

        for line in value.split('\n') {
            let line = line.trim().to_string();

            /*
            // separate line is checked for width so that ansi codes aren't counted
            let mut length_check_line = line.clone();
            if length_check_line.starts_with("\x1b") {
                // spaces are used to indent items, so they can be used as a marker of the end of an
                // ansi code
                while !(length_check_line.starts_with('H') || length_check_line.starts_with(' ')) {
                    length_check_line.remove(0);
                }
                
                if length_check_line.starts_with('H') {
                    length_check_line.remove(0);
                }
            }

            while length_check_line.contains("\x1b") { length_check_line.pop(); }

            if length_check_line.len() > width {
                width = length_check_line.len();
            }
            */

            let len = line_length(line.clone());
            if len > width {
                width = len;
            }

            lines.push(line);
        }

        Self {
            lines,
            width: width as u32,
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
                    // store escapes while other characters are trimmed so that styling is preserved
                    let mut escapes = String::new();
                    while line.contains('\x1b') { escapes.push(line.pop().unwrap()); }

                    while line.len() > (max_width - 3) as usize { line.pop(); }
                    line.push_str("...");

                    while !escapes.is_empty() { line.push(escapes.pop().unwrap()); }
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

/// Get the line length after removing ANSI codes
pub fn line_length(mut line: String) -> usize {
    if line.starts_with("\x1b") {
        // spaces are used to indent items, so they can be used as a marker of the end of an
        // ansi code
        while !(line.starts_with('H') || line.starts_with(' ')) {
            line.remove(0);
        }
        
        if line.starts_with('H') {
            line.remove(0);
        }
    }

    while line.contains('\x1b') { line.pop(); }

    line.len()
}
