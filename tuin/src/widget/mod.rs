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
        }
    }
}
