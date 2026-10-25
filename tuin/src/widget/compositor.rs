//! # compositor
//! This module holds rendering logic for compositing widgets.

use crate::*;

/// # CompositorType
/// This enum differentiates between a vertical or horizontal compositor.
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum CompositorType {
    Row,
    Column,
}

impl CompositorType {
    /// Handle rendering of a compositor's children
    pub fn render(self, children: &Vec<Widget>, context: RenderContext) -> Render {
        match self {
            Self::Row => {
                let mut render = Render::new(self.render_row(children, context), context);
                render.context.max_width = None;
                render
            },
            Self::Column => Render::new(self.render_column(children, context), context),
        }
    }

    /// Handle rendering of a row's children. Returns a `String`, which will be converted to a
    /// `Render` later.
    pub fn render_row(self, children: &Vec<Widget>, mut context: RenderContext) -> String {
        let mut buffer = String::new();

        // Fix max width
        let number_of_children = children.len() as u32;
        if let Some(mut max_width) = context.max_width {
            max_width -= (number_of_children * context.row_pad) - 1; // padding
            max_width /= number_of_children;
            context.max_width = Some(max_width);
            eprintln!("max width is {max_width}");
        }

        for child in children {
            let render = child.render(context);
            context.x += render.width() + context.row_pad;

            buffer.push_str(<Render as Into<String>>::into(render).as_str());

            //x = context.x;
            //buffer.push_str(format!("\x1b[{y};{x}H").as_str());
            buffer.push_str("  ");
        }

        buffer.pop();

        buffer
    }

    /// Handle rendering of a column's children. Returns a `String`, which will be converted to a
    /// `Render` later.
    pub fn render_column(self, children: &Vec<Widget>, mut context: RenderContext) -> String {
        let mut buffer = String::new();

        for child in children {
            let render = child.render(context);
            context.y += render.height();

            buffer.push_str(<Render as Into<String>>::into(render).as_str());

            buffer.push('\n');
        }

        buffer.pop();

        buffer
    }
}

impl Widget {
    /// Add a child widget to this one, panicking if self isn't a compositor.
    pub fn add_child(&mut self, child: Self) {
        let Self::Compositor {
            children,
            compositor_type,
        } = self
        else {
            panic!("Widget<T>::add_child(): Non-compositing widgets can't have children.");
        };

        children.push(child);

        *self = Self::Compositor {
            children: children.clone(),
            compositor_type: compositor_type.clone(),
        };
    }
}
