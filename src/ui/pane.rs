use tuin::*;

use super::item::Item;
use crate::{logging::log, state::*, ui::DROPDOWN_ITEM_NAMES};

use std::{
    fs::{metadata, read_dir},
    path::PathBuf,
    sync::mpsc::{Receiver, Sender, channel},
    thread::spawn,
};

/// One pane in the UI
#[derive(Debug)]
pub struct Pane {
    pub path: Box<PathBuf>,
    pub items: Vec<Item>,
    hovered_item_index: usize,
    pub hovered: bool,
    pub first_row: usize,
    pub typing_buffer: Option<TypingBuffer>,
    search_receiver: Option<Receiver<Item>>,
    search_pattern: Option<String>,
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
            typing_buffer: None,
            search_receiver: None,
            search_pattern: None,
        };
        new.populate();
        new
    }

    /// Build a column of widgets to represent this Pane.
    pub fn build(&self) -> Widget {
        let mut column = col![Widget::Label(self.title()), Widget::Pad];

        let mut last_row = self.first_row + height() - 2;
        if last_row > self.items.len() {
            last_row = self.items.len();
        }

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

        // skip clearing items if currently receiving search results
        if let Some(receiver) = &mut self.search_receiver {
            let hover_first = self.items.is_empty();
            while let Ok(item) = receiver.try_recv() {
                self.items.push(item);
            }

            if hover_first && !self.items.is_empty() {
                self.hovered_item_index = 0;
                self.items[0].hovered = true;
            }

            return;
        }

        // store current item in case the same file still exists, to be able to keep context menus
        // open
        let hovered_item = match self.hovered_item() {
            Some(item) => Some(item.clone()),
            None => None,
        };

        self.items.clear();

        let Ok(files) = read_dir(*self.path.clone()) else {
            return;
        };

        let mut should_set_index = true;

        for file in files {
            if let Ok(entry) = file {
                let name = format_path(entry.path());

                // check if it's the currently hovered Item
                if let Some(hovered) = &hovered_item && hovered.name() == name {
                    self.items.push(hovered.clone());
                } else {
                    self.items.push(Item::new(&name, entry.path().is_dir()));
                }

                if name == hovered_name {
                    self.hovered_item_index = self.items.len() - 1;
                    should_set_index = false;
                }
            }
        }

        if should_set_index && !self.items.is_empty() && self.hovered_item_index >= self.items.len()
        {
            self.hovered_item_index = 0;
            self.first_row = 0;
        }
        let buffer = self.typing_buffer.is_some();

        if let Some(item) = self.hovered_item() && !buffer {
            item.hovered = true;
        }
    }

    /// Move the highlight down one, if possible.
    pub fn move_highlight_down(&mut self) {
        if let Some(item) = self.hovered_item()
            && let Some(index) = &mut item.expanded_index
        {
            *index += 1;
            if *index > DROPDOWN_ITEM_NAMES.len() {
                item.expanded_index = None;
            } else {
                return;
            }
        }

        if !self.items.is_empty() && self.hovered_item_index < self.items.len() - 1 {
            if self.first_row + height() - 3 <= self.hovered_item_index {
                log(format!(
                    "increasing row because hovered_item_index ({}) >= height - 3 ({})",
                    self.hovered_item_index,
                    height()
                ));
                self.first_row += 1;
            }
            self.hovered_item().unwrap().hovered = false;
            self.hovered_item_index += 1;
            self.hovered_item().unwrap().hovered = true;
        }
    }

    /// Move the highlight up one, if possible.
    pub fn move_highlight_up(&mut self) {
        if let Some(item) = self.hovered_item()
            && let Some(index) = &mut item.expanded_index
        {
            if *index == 0 {
                item.expanded_index = None;
            } else {
                *index -= 1;
                return;
            }
        }

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

    pub fn update_typing(&mut self, input: Input) -> Option<UpdateItemResult> {
        if let Some(buffer) = &mut self.typing_buffer {
            let Some(result) = buffer.take_input(input) else {
                return None;
            };
            match result {
                BufferInputResult::Cancel => {
                    self.typing_buffer = None;
                    self.items[self.hovered_item_index].hovered = true;
                    Some(UpdateItemResult::SetCursorState(CursorState::Hovering))
                }
                BufferInputResult::Complete(text) => self.finish_buffer(text),
            }
        } else {
            let path = self.hovered_item_path();
            self.hovered_item().unwrap().input_typing(input, path)
        }
    }

    pub fn stop_searching(&mut self) {
        self.typing_buffer = None;
        self.search_receiver = None;
        self.search_pattern = None;
    }

    fn finish_buffer(&mut self, text: String) -> Option<UpdateItemResult> {
        let Some(buffer) = &mut self.typing_buffer else {
            unreachable!();
        };

        match buffer.location() {
            TypingLocation::Goto => {
                *self.path = PathBuf::from(text);
                self.typing_buffer = None;
                Some(UpdateItemResult::DoBoth(CursorState::Hovering))
            }
            // start a file search in a background thread
            TypingLocation::Search => {
                self.search_pattern = Some(text.clone());
                let (sender, receiver) = channel();
                let path = *self.path.clone();
                self.items.clear();
                spawn(|| search(sender, path, text, true));

                self.search_receiver = Some(receiver);
                self.typing_buffer = None;
                Some(UpdateItemResult::DoBoth(CursorState::Hovering))
            }
            _ => unreachable!(),
        }
    }

    fn title(&self) -> String {
        if let Some(buffer) = &self.typing_buffer {
            if buffer.location() == TypingLocation::Goto {
                format!("Go to: \x1b[1;31m{}", buffer.buffer())
            } else {
                format!("{}\n\nFind: \x1b[1;31m{}\x1b[0m", self.display_path(), buffer.buffer())
            }
        } else {
            let path = self.display_path();
            if let Some(pattern) = &self.search_pattern {
                format!("{}\n\nFind: \x1b[1;31m{}\x1b[0m", path, pattern)
            } else {
                path
            }
        }
    }

    fn display_path(&self) -> String {
        let mut path = self
            .path
            .clone()
            .into_string()
            .unwrap_or("[couldn't get path]".to_string())
            .replace("/", "/\n");
        path.remove(1);
        path.trim().to_string()
    }
}

// the background thread that searches for files
fn search(sender: Sender<Item>, dir: PathBuf, pattern: String, is_parent: bool) {
    let Ok(files) = read_dir(dir.clone()) else {
        return;
    };

    for file in files {
        let Ok(file) = file else {
            continue;
        };

        let name = file
            .path()
            .into_string()
            .unwrap_or("[couldn't read name]".to_string())
            .to_string();
        let last_segment = format_path(file.path());
        if last_segment.contains(&pattern) {
            log(format!("ui::pane::search(): Found file {name}"));
            if let Err(e) = sender.send(Item::new(name, file.path().is_dir())) {
                log(format!(
                    "ui::pane::search(): Couldn't send item because {e}"
                ));
            }
        }

        if let Ok(metadata) = metadata(file.path())
            && metadata.file_type().is_dir()
        {
            let mut dir = dir.clone();
            dir.push(last_segment);
            search(sender.clone(), dir, pattern.clone(), false);
        }
    }

    if is_parent {
        log("ui::pane::search(): Finished search")
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
