//! # render
//! This module holds the `Render` struct, which is how rendered widgets are stored.

/// # Render
/// A rendered widget, split into lines.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Render {
    lines: Vec<String>,
    width: u32,
    context: RenderContext,
}

impl Render {
    /// Construct a new `Render` with a position context and a `String`.
    pub fn new(value: String, context: RenderContext) -> Self {
        let mut lines = Vec::new();
        let mut width = 0;

        for line in value.split('\n') {
            let mut line = line.trim().to_string();

            if line.starts_with("\x1b") {
                while !line.starts_with('H') {
                    line.remove(0);
                }
                line.remove(0);
            }

            if line.len() > width {
                width = line.len();
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
            // jump to the correct x coordinate
            buffer.push_str(format!("\x1b[{y};{x}H").as_str());

            if line.len() > value.context.max_width as usize {
                while line.len() > value.context.max_width as usize - 3 { line.pop(); }
                line.push_str("...");
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
    pub max_width: u32,
}

impl Default for RenderContext {
    fn default() -> Self {
        let width = unsafe { crate::size::terminal_width() } as u32;
        Self {
            x: 1,
            y: 1,
            terminal_width: width,
            max_width: width,
        }
    }
}
