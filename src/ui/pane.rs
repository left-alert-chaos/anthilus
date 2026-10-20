use tuin::*;

use std::{path::PathBuf, fs::read_dir};

/// One pane in the UI
#[derive(Clone, Debug)]
pub struct Pane {
    path: Box<PathBuf>,
    items: Vec<super::item::Item>,
    highlighted_item_index: usize,
}

impl Pane {
    /// Create a new Pane with the given directory.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let mut new = Self {
            path: Box::new(dir.into()),
            items: vec![],
            highlighted_item_index: 0,
        };
        new.populate();
        new
    }

    /// Build a column of widgets to represent this Pane.
    pub fn build(&self) -> Widget {
        let mut column = col![
            Widget::Label(self.path.to_str().unwrap_or("[unknown path]").to_string()),
            Widget::Pad
        ];

        for item in &self.items {
            column.add_child(item.build());
        }

        column
    }

    /// Remove existing items and replace with up-to-date ones.
    pub fn populate(&mut self) {
        self.items.clear();
        
        let Ok(files) = read_dir(*self.path.clone()) else {
            return;
        };

        for file in files {
            if let Ok(entry) = file {
                let name = format!("{:?}", entry.path());
                self.items.push(super::Item::new(&name));
            }
        }

        self.highlighted_item_index = self.items.len() - 1;
        self.items[self.highlighted_item_index].hovered = true;
    }

    /// Move the highlight down one, if possible.
    pub fn move_highlight_down(&mut self) {
        if self.highlighted_item_index < self.items.len() - 1 {
            self.items[self.highlighted_item_index].hovered = false;
            self.highlighted_item_index += 1;
            self.items[self.highlighted_item_index].hovered = true;
        }
    }

    /// Move the highlight up one, if possible.
    pub fn move_highlight_up(&mut self) {
        if self.highlighted_item_index > 0 {
            self.items[self.highlighted_item_index].hovered = false;
            self.highlighted_item_index -= 1;
            self.items[self.highlighted_item_index].hovered = true;
        }
    }
}
