use crate::{
    logging::log,
    state::{BufferInputResult, CursorState, TypingBuffer, TypingLocation, UpdateItemResult},
    render::*,
    state::input::Input,
};

use std::{fs, path::PathBuf};

/// The names of the options shown when an item is expanded
pub static DROPDOWN_ITEM_NAMES: [&str; 4] = ["Open", "Open in new pane", "Delete", "Rename"];

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
        let name = name.into();
        Self {
            name: name.into(),
            expanded_index: None,
            hovered: false,
            typing_buffer: None,
            is_dir,
            is_preview_line,
        }
    }

    pub fn render(&self, pane_is_hovered: bool, context: &mut RenderContext) -> Render {
        let mut text = if let Some(buffer) = &self.typing_buffer {
            format!("{} -> {}", self.name, buffer.display_buffer())
        } else {
            self.name()
        };

        if text.len() > context.width as usize {
            while text.len() > context.width as usize - 3 { text.pop(); }
            text.push_str("...");
        }

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
        text.insert_str(0, format!("\x1b[{};{}H", context.y, context.x).as_str());

        if let Some(expanded_index) = self.expanded_index {
            for (index, name) in DROPDOWN_ITEM_NAMES.iter().enumerate() {
                let mut name = name.to_string();
                context.y += 1;
                if expanded_index > 0 && expanded_index - 1 == index && pane_is_hovered {
                    name.insert_str(0, "\x1b[1;31m ");
                }
                text = format!("{text}\n\x1b[{};{}H  {name}\x1b[0m", context.y, context.x);
            }
        }

        Render::new(text)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns whether or not to repopulate pane after item performs action
    pub fn select(&mut self, path: PathBuf) -> Option<UpdateItemResult> {
        if self.is_preview_line {
            return None;
        }

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
                log("item::Item::input_typing(): Canceling buffer");
                Some(UpdateItemResult::SetCursorState(CursorState::Hovering))
            }
            BufferInputResult::Complete(string) => {
                self.rename(path, string);
                Some(UpdateItemResult::RepopulateAndHover)
            }
        }
    }

    fn rename(&self, path: PathBuf, string: String) {
        let mut file_parent = path.clone();
        file_parent.pop();
        file_parent.push(string);

        if let Err(e) = fs::rename(path, file_parent) {
            log(format!(
                "Item::input_typing(): Couldn't rename item {}; generated error {e}",
                self.name
            ));
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
            "Open" => Some(UpdateItemResult::Goto),
            "Open in new pane" => Some(UpdateItemResult::NewPane),
            _ => {
                log(format!(
                    "Item::select_dropdown(): DROPDOWN_ITEM_NAMES[{index}] is {}, which isn't a handled action! Something is seriously wrong!",
                    DROPDOWN_ITEM_NAMES[index]
                ));
                None
            }
        }
    }

    fn delete(&self, path: PathBuf) {
        if self.is_dir {
            self.delete_dir(path);
            return;
        }
        if let Err(e) = fs::remove_file(path) {
            log(format!(
                "Item::delete(): couldn't delete item {}; generated error {e}",
                self.name,
            ));
        }
    }

    fn delete_dir(&self, path: PathBuf) {
        if let Err(e) = fs::remove_dir_all(path) {
            log(format!(
                "Item::delete(): couldn't delete directory item {}; generated error {e}",
                self.name,
            ));
        }
    }

    fn start_rename(&mut self) {
        self.typing_buffer = Some(TypingBuffer::new(self.name.clone(), TypingLocation::Rename));
    }
}
