//! # helpers
//! This module holds some helpers to aid in creating widgets.

use crate::Widget;

#[macro_export]
macro_rules! col {
    ( $( $x:expr ),* ) => {
        {
            let mut column: tuin::Widget = tuin::Widget::new_compositor(tuin::CompositorType::Column);
            $(
                column.add_child($x);
            )*
            column
        }
    };
}

#[macro_export]
macro_rules! row {
    ( $( $x:expr ),* ) => {
        {
            let mut compositor = tuin::Widget::new_compositor(tuin::CompositorType::Row);
            $(
                compositor.add_child($x);
            )*
            compositor
        }
    }
}

/// Create a new `Center` widget
/// A very small function.
pub fn center(child: Widget) -> Widget {
    Widget::Center(Box::new(child))
}
