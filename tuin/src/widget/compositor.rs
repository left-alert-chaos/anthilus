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
    where T: Clone + fmt::Debug
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
    where T: Clone + fmt::Debug
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
            buffer.push_str(format!("\x1b[{x};{y}H").as_str());
        }

        buffer
    }

    /// Handle rendering of a column's children. Returns a `String`, which will be converted to a
    /// `Render` later.
    pub fn render_column<T>(self, children: &Vec<Widget<T>>, context: RenderContext) -> String
    where T: Clone + fmt::Debug
    {
        let mut buffer = String::new();
        let mut x;
        let mut y;

        for child in children {
            let render = child.render(context);
            context.y += render.height();

            buffer.push_str(<Render as Into<String>>::into(render).as_str());

            x = context.x;
            y = context.y;
            buffer.push_str(format!("\x1b[{x};{y}H").as_str());
        }

        buffer.pop();

        buffer
    }
}

impl<T> Widget<T>
where T: Clone + fmt::Debug
{
    /// Add a child widget to this one, panicking if self isn't a compositor.
    pub fn add_child(&mut self, child: Self) {
        let Self::Compositor { children, compositor_type } = self else {
            panic!("Widget<T>::add_child(): Non-compositing widgets can't have children.");
        };

        children.push(child);

        *self = Self::Compositor { children: children.clone(), compositor_type: compositor_type.clone() };
    }    
}
