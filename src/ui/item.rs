use tuin::*;

use crate::{
    logging::log,
    state::{BufferInputResult, CursorState, TypingBuffer, TypingLocation, UpdateItemResult},
};

use std::{fs, path::PathBuf};

/// The names of the options shown when an item is expanded
pub static DROPDOWN_ITEM_NAMES: [&str; 2] = ["Delete", "Rename"];

#[derive(Clone, Debug)]
pub struct Item {
    name: String,
    pub expanded_index: Option<usize>,
    pub hovered: bool,
    typing_buffer: Option<TypingBuffer>,
    is_dir: bool,
    pub is_preview_line: bool,
}

impl Item {
    pub fn new(name: impl Into<String>, is_dir: bool, is_preview_line: bool) -> Self {
        Self {
            name: name.into(),
            expanded_index: None,
            hovered: false,
            typing_buffer: None,
            is_dir,
            is_preview_line,
        }
    }

    pub fn build(&self, pane_is_hovered: bool) -> Widget {
        let mut text = if let Some(buffer) = &self.typing_buffer {
            buffer.buffer()
        } else {
            self.name()
        };

        if !self.is_preview_line {
            if self.is_dir {
                text.insert_str(0, " ");
            } else {
                text.insert_str(0, " ");
            }
        }

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

            if let Some(Widget::Label(string)) = main.child(index)
                && index != 0
            {
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
    pub fn select(&mut self, path: PathBuf) -> Option<UpdateItemResult> {
        if self.is_preview_line { return None; }

        if let Some(index) = self.expanded_index {
            match index {
                0 => {
                    self.expanded_index = None;
                    None
                }
                _ => self.select_dropdown(path),
            }
        } else {
            self.expanded_index = Some(0);
            None
        }
    }

    /// Process some Input while typing
    pub fn input_typing(&mut self, input: Input, path: PathBuf) -> Option<UpdateItemResult> {
        let Some(buffer) = &mut self.typing_buffer else {
            return None;
        };

        let Some(result) = buffer.take_input(input) else {
            return None;
        };
        self.typing_buffer = None;
        match result {
            BufferInputResult::Cancel => {
                Some(UpdateItemResult::SetCursorState(CursorState::Hovering))
            }
            BufferInputResult::Complete(string) => {
                let mut file_parent = path.clone();
                file_parent.pop();
                file_parent.push(string);
                if let Err(e) = fs::rename(path, file_parent) {
                    log(format!(
                        "Item::input_typing(): Couldn't rename item {}; generated error {e}",
                        self.name
                    ));
                }
                Some(UpdateItemResult::DoBoth(CursorState::Hovering))
            }
        }
    }

    fn select_dropdown(&mut self, path: PathBuf) -> Option<UpdateItemResult> {
        let Some(mut index) = self.expanded_index else {
            return None;
        };
        index -= 1;

        match DROPDOWN_ITEM_NAMES[index] {
            "Delete" => {
                self.delete(path);
                Some(UpdateItemResult::Repopulate)
            }
            "Rename" => {
                self.start_rename();
                Some(UpdateItemResult::SetCursorState(CursorState::Typing))
            }
            _ => {
                log(format!(
                    "Item::select_dropdown(): DROPDOWN_ITEM_NAMES[{index}] is {}, which isn't a handled action! Something is seriously wrong!",
                    DROPDOWN_ITEM_NAMES[index]
                ));
                None
            }
        }
    }

    fn delete(&mut self, path: PathBuf) {
        if let Err(e) = fs::remove_file(path) {
            log(format!(
                "Item::delete(): couldn't delete item {}; generated error {e}",
                self.name
            ));
        }
    }

    fn start_rename(&mut self) {
        self.typing_buffer = Some(TypingBuffer::new(TypingLocation::Rename));
    }
}
