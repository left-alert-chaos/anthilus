use tuin::*;

use crate::{logging::log, state::{UpdateItemResult, CursorState}};

use std::{fs, path::PathBuf};

/// The names of the options shown when an item is expanded
pub static DROPDOWN_ITEM_NAMES: [&str; 2] = ["Delete", "Rename"];

#[derive(Clone, Debug)]
pub struct Item {
    name: String,
    pub expanded_index: Option<usize>,
    pub hovered: bool,
    typing_buffer: Option<TypingBuffer>,
}

impl Item {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            expanded_index: None,
            hovered: false,
            typing_buffer: None,
        }
    }

    pub fn build(&self, pane_is_hovered: bool) -> Widget {
        let mut text = if let Some(buffer) = &self.typing_buffer {
            buffer.buffer.clone()
        } else {
            self.name()
        };

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
    pub fn select(&mut self, path: PathBuf) -> Option<UpdateItemResult> {
        if let Some(index) = self.expanded_index {
            match index {
                0 => {
                    self.expanded_index = None;
                    None
                }
                _ => self.select_dropdown(path)
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
        match result {
            BufferInputResult::Cancel => {
                self.typing_buffer = None;
                Some(UpdateItemResult::SetCursorState(CursorState::Hovering))
            }
            BufferInputResult::Complete(string) => {
                let mut file_parent = path.clone();
                file_parent.pop();
                file_parent.push(string);
                if let Err(e) = fs::rename(path, file_parent) {
                    log(format!("Item::input_typing(): Couldn't rename item {}; generated error {e}", self.name));
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
            "Delete" => { self.delete(path); Some(UpdateItemResult::Repopulate) },
            "Rename" => { self.start_rename(); Some(UpdateItemResult::SetCursorState(CursorState::Typing)) }
            _ => { log(format!("Item::select_dropdown(): DROPDOWN_ITEM_NAMES[{index}] is {}, which isn't a handled action! Something is seriously wrong!", DROPDOWN_ITEM_NAMES[index])); None },
        }
    }

    fn delete(&mut self, path: PathBuf) {
        if let Err(e) = fs::remove_file(path) {
            log(format!("Item::delete(): couldn't delete item {}; generated error {e}", self.name));
        }
    }

    fn start_rename(&mut self) {
        self.typing_buffer = Some(TypingBuffer::new(TypingLocation::Rename));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TypingLocation {
    Rename,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct TypingBuffer {
    buffer: String,
    cursor: usize,
    location: TypingLocation,
}

impl TypingBuffer {
    fn new(location: TypingLocation) -> Self {
        Self {
            buffer: String::new(),
            cursor: 0,
            location,
        }
    }

    /// Take some input and return whether to use the buffer, cancel operation, or wait
    fn take_input(&mut self, input: Input) -> Option<BufferInputResult> {
        match input {
            Input::Character(c) => self.buffer.push(c),
            Input::Escape => return Some(BufferInputResult::Cancel),
            Input::Return => return Some(BufferInputResult::Complete(self.buffer.clone())),
            Input::Backspace => { self.buffer.pop(); }
            _ => {}
        }

        None
    }
}

/// What to do, if anything, after a `TypingBuffer` takes input
#[derive(Clone, PartialEq, Eq, Debug)]
enum BufferInputResult {
    Complete(String),
    Cancel,
}
