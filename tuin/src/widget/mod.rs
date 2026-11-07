//! # widget
//! This is logic for all widgets.

mod compositor;

use crate::*;
pub use compositor::CompositorType;

/// # Widget
/// This is the base for all UI elements. This enum is both standard widgets and compositors.
/// T is a Message that some widgets emit when interacted with.
#[derive(Eq, PartialEq, Clone)]
pub enum Widget {
    /// A piece of text
    Label(String),

    /// A widget that holds other widgets
    Compositor {
        children: Vec<Self>,
        compositor_type: CompositorType,
    },

    /// A widget that displays nothing, used as padding between other widgets in a column.
    Pad,

    /// A widget that holds one child and centers it based on maximum width. Center-justifies all
    /// lines of child.
    Center(Box<Self>),
}

impl Widget {
    /// Create a new compositor with the given layout type
    pub fn new_compositor(compositor_type: CompositorType) -> Self {
        Self::Compositor {
            children: Vec::new(),
            compositor_type,
        }
    }

    /// Render the widget into a `Render` object
    pub fn render(&self, context: RenderContext) -> Render {
        match self {
            Self::Label(text) => Render::new(text.clone(), context),
            Self::Compositor {
                children,
                compositor_type,
            } => compositor_type.render(children, context),
            Self::Pad => {
                Render::new(String::new(), context)
            }
            Self::Center(child) => child.render_center(context),
        }
    }

    fn render_center(&self, context: RenderContext) -> Render {
        let mut render = self.render(context);
        let Some(max_width) = context.max_width else { return render; };

        let mut gutter = String::new();

        for line in render.lines() {
            let diff = (max_width - line_length(line.clone()) as u32) / 2;
            for _ in 0..diff { gutter.push(' '); }
            line.insert_str(0, gutter.as_str());
        }

        render
    }
}
