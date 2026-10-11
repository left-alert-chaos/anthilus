//! # compositor
//! This module holds rendering logic for compositing widgets.

use crate::*;

use std::fmt;

/// # CompositorType
/// This enum differentiates between a vertical or horizontal compositor.
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum CompositorType {
    Row,
    Column,
}

impl CompositorType
{
    /// Handle rendering of a compositor's children
    pub fn render<T>(self, children: &Vec<Widget<T>>, context: RenderContext) -> Render
    where T: fmt::Debug
    {
        let text = match self {
            Self::Row => self.render_row(children, context),
            Self::Column => self.render_column(children, context),
        };
        Render::new(text, context)
    }

    /// Handle rendering of a row's children. Returns a `String`, which will be converted to a
    /// `Render` later.
    pub fn render_row<T>(self, children: &Vec<Widget<T>>, mut context: RenderContext) -> String
    where T: fmt::Debug
    {
        let mut buffer = String::new();
        let mut x;
        let mut y;

        for child in children {
            let render = child.render(context);
            context.x += render.width() + 1;

            buffer.push_str(<Render as Into<String>>::into(render).as_str());
            
            x = context.x;
            y = context.y;
            print!("\x1b[{x};{y}H");
        }

        buffer
    }

    /// Handle rendering of a column's children. Returns a `String`, which will be converted to a
    /// `Render` later.
    pub fn render_column<T>(self, children: &Vec<Widget<T>>, context: RenderContext) -> String
    where T: fmt::Debug
    {
        let mut buffer = String::new();

        for child in children {
            let render = child.render(context);
            buffer.push_str(<Render as Into<String>>::into(render).as_str());
            buffer.push('\n');
        }

        buffer.pop();

        buffer
    }
}
