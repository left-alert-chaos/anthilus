use tuin::*;

use crate::logging::log;

use std::fs;

/// The names of the options shown when an item is expanded
pub static DROPDOWN_ITEM_NAMES: [&str; 1] = ["Delete"];

#[derive(Clone, Debug)]
pub struct Item {
    name: String,
    pub expanded_index: Option<usize>,
    pub hovered: bool,
}

impl Item {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            expanded_index: None,
            hovered: false,
        }
    }

    pub fn build(&self, pane_is_hovered: bool) -> Widget {
        let mut text = self.name.clone();

        // invert colors if hovered
        if self.hovered && pane_is_hovered {
            text.insert_str(0, "\x1b[1;31m ");
        }
        text.push_str("\x1b[0m");

        let mut main = Widget::Label(text);

        if let Some(index) = self.expanded_index {
            main = col![main];

            for name in DROPDOWN_ITEM_NAMES {
                main.add_child(Widget::Label(format!("  {name}")));
            }

            if let Some(Widget::Label(string)) = main.child(index) && index != 0 {
                string.insert_str(0, "\x1b[1;31m ");
                string.push_str("\x1b[0m");
            }
        }

        main
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns whether or not to repopulate pane after item performs action
    pub fn select(&mut self) -> bool {
        if self.expanded_index.is_some() {
            if self.expanded_index == Some(0) {
                self.expanded_index = None;
                false
            } else {
                self.select_dropdown()
            }
        } else {
            self.expanded_index = Some(0);
            false
        }
    }

    fn select_dropdown(&mut self) -> bool {
        let Some(mut index) = self.expanded_index else {
            return false;
        };
        index -= 1;

        match DROPDOWN_ITEM_NAMES[index] {
            "Delete" => { self.delete(); true },
            _ => { log(format!("Item::select_dropdown(): DROPDOWN_ITEM_NAMES[{index}] is {}, which isn't a handled action! Something is seriously wrong!", DROPDOWN_ITEM_NAMES[index])); false },
        }
    }

    fn delete(&mut self) {
        if let Err(e) = fs::remove_file(self.name.clone()) {
            log(format!("Item::delete(): couldn't delete item {}; generated error {e}", self.name));
        }
    }
}
