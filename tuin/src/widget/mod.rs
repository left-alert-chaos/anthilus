//! # widget
//! This is logic for all widgets.

mod compositor;

use compositor::CompositorType;

use std::fmt;

/// # Widget<T>
/// This is the base for all UI elements. This enum is both standard widgets and compositors.
/// T is a Message that some widgets emit when interacted with.
#[derive(Eq, PartialEq, Clone)]
pub enum Widget<T>
where T: fmt::Debug
{
    Label(String),
    Compositor {
        children: Vec<Self>,
        compositor_type: CompositorType,
    },
    Button(String, T),
}

impl<T> Widget<T>
where T: fmt::Debug
{
}
