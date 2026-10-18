use tuin::*;

use std::path;

/// One pane in the UI
#[derive(Clone, Debug)]
pub struct Pane {
    path: Box<path::Path>, // path path path path path path path path path path    
    items: Vec<super::item::Item>,
    highlighted_item: usize,
}

impl Pane {
    /// Build a column of widgets to represent this Pane.
    pub fn build(&self) -> Widget {
        let mut column = Widget::new_compositor(CompositorType::Column);

        for item in &self.items {
            column.add_child(item.build());
        }

        column
    }
}
