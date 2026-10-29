use tuin::*;

use super::item::Item;
use crate::logging::log;

use std::{fs::read_dir, path::PathBuf};

/// One pane in the UI
#[derive(Clone, Debug)]
pub struct Pane {
    pub path: Box<PathBuf>,
    items: Vec<Item>,
    hovered_item_index: usize,
    pub hovered: bool,
    pub first_row: usize,
}

impl Pane {
    /// Create a new Pane with the given directory.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let mut new = Self {
            path: Box::new(dir.into()),
            items: vec![],
            hovered_item_index: 0,
            hovered: true,
            first_row: 0,
        };
        new.populate();
        new
    }

    /// Build a column of widgets to represent this Pane.
    pub fn build(&self) -> Widget {
        let mut column = col![
            Widget::Label(format_path(*self.path.clone())),
            Widget::Pad
        ];

        let mut last_row = self.first_row + height() - 2;
        if last_row > self.items.len() { last_row = self.items.len(); }

        for item in &self.items[self.first_row..last_row] {
            column.add_child(item.build(self.hovered));
        }

        column
    }

    /// Remove existing items and replace with up-to-date ones.
    pub fn populate(&mut self) {
        let hovered_name = if let Some(item) = self.hovered_item() {
            item.name()
        } else {
            String::from("[unknown file name]")
        };

        self.items.clear();

        let Ok(files) = read_dir(*self.path.clone()) else {
            return;
        };

        let mut should_set_index = true;

        for file in files {
            if let Ok(entry) = file {
                let name = format_path(entry.path());
                self.items.push(super::Item::new(&name));

                if name == hovered_name {
                    self.hovered_item_index = self.items.len() - 1;
                    should_set_index = false;
                }
            }
        }

        if should_set_index && !self.items.is_empty() {
            self.hovered_item_index = 0;
            self.first_row = 0;
        }
        if let Some(item) = self.hovered_item() {
            item.hovered = true;
        }
    }

    /// Move the highlight down one, if possible.
    pub fn move_highlight_down(&mut self) {
        if !self.items.is_empty() && self.hovered_item_index < self.items.len() - 1 {
            if self.first_row + height() - 3 <= self.hovered_item_index {
                log(format!("increasing row because hovered_item_index ({}) >= height - 3 ({})", self.hovered_item_index, height()));
                self.first_row += 1;
            }
            self.hovered_item().unwrap().hovered = false;
            self.hovered_item_index += 1;
            self.hovered_item().unwrap().hovered = true;
        }
    }

    /// Move the highlight up one, if possible.
    pub fn move_highlight_up(&mut self) {
        if self.hovered_item_index > 0 {
            if self.hovered_item_index == self.first_row && self.first_row > 0 {
                self.first_row -= 1;
            }
            self.hovered_item().unwrap().hovered = false;
            self.hovered_item_index -= 1;
            self.hovered_item().unwrap().hovered = true;
        }
    }

    /// Get the path of the currently hovered item
    pub fn hovered_item_path(&mut self) -> PathBuf {
        let mut path = *self.path.clone();
        if let Some(item) = self.hovered_item() {
            path.push(item.name());
        }
        path
    }

    pub fn hovered_item(&mut self) -> Option<&mut Item> {
        if !self.items.is_empty() && self.hovered_item_index < self.items.len() {
            Some(&mut self.items[self.hovered_item_index])
        } else {
            None
        }
    }
}

fn format_path(path: PathBuf) -> String {
    path.into_string()
        .unwrap_or(String::from("[couldn't decode name]"))
        .split("/")
        .into_iter()
        .last()
        .unwrap_or("[couldn't decode name]")
        .to_string()
}
