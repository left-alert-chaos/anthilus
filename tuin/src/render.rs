//! # render
//! This module holds the `Render` struct, which is how rendered widgets are stored.

/// # Render
/// A rendered widget, split into lines.
#[derive(PartialEq, Eq, Clone)]
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
            lines.push(line.to_string());
            if line.len() > width {
                width = line.len();
            }
        }

        Self {
            lines,
            width: width as u32,
            context
        }
    }
}

impl Render {
    /// Get the width of a `Render`.
    pub fn width(&self) -> u32 {
        self.width
    }
}

impl From<Render> for String {
    /// Convert the `Render` into a string with ANSI codes. Final step of the render pipeline.
    fn from(value: Render) -> Self {
        let mut buffer = String::new();
        let x = value.context.x;

        for line in value.lines {
            // jump to the correct x coordinate
            buffer.push_str(format!("\x1b[;{x}H").as_str());
            buffer.push_str(line.as_str());
        }

        buffer
    }
}

/// # RenderContext
/// Holds size and location information that a `Render` uses to draw into a `String`.
#[derive(Clone, Copy, Eq, PartialEq)]
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
