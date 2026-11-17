use super::item::Item;
use crate::{
    logging::log, render::*, size::height, state::input::Input, state::*, ui::DROPDOWN_ITEM_NAMES,
};

use std::{
    fs::{create_dir_all, metadata, read, read_dir, write}, path::PathBuf, sync::mpsc::{Receiver, Sender, channel}, thread::spawn,
};

/// What purpose the pane is serving
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PaneType {
    Preview,
    Directory,
}

/// One pane in the UI
#[derive(Debug)]
pub struct Pane {
    pub path: Box<PathBuf>,
    pub items: Vec<Item>,
    pub hovered_item_index: usize,
    pub hovered: bool,
    pub first_row: usize,
    pub typing_buffer: Option<TypingBuffer>,
    search_receiver: Option<Receiver<Item>>,
    search_pattern: Option<String>,
    pane_type: PaneType,
}

impl Pane {
    /// Create a new Pane with the given directory.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();
        let mut new = Self {
            path: Box::new(dir.clone()),
            items: vec![],
            hovered_item_index: 0,
            hovered: true,
            first_row: 0,
            typing_buffer: None,
            search_receiver: None,
            search_pattern: None,
            pane_type: if dir.is_dir() {
                PaneType::Directory
            } else {
                PaneType::Preview
            },
        };
        new.populate();
        new
    }

    pub fn render(&mut self, context: &mut RenderContext) -> Render {
        let mut buffer = format!("\x1b[1;{}H{}", context.x, self.title(&*context).as_str());

        let last_row = self.last_row();
        let mut first_row = self.first_row;

        if let Some(item) = self.hovered_item() && item.expanded_index.is_some() && self.items.len() + DROPDOWN_ITEM_NAMES.len() > self.maximum_row() {
            first_row += DROPDOWN_ITEM_NAMES.len();
        }

        context.y += self.title_height() as u32;

        for item in &self.items[first_row..last_row] {
            buffer.push_str(item.render(self.hovered, context).text.as_str());
            context.y += 1;
        }

        // name of file being created
        if let Some(typing_buffer) = &self.typing_buffer && self.is_creating_file_or_directory() {
            let mut name = typing_buffer.display_buffer();
            if typing_buffer.location() == TypingLocation::NewDirectory {
                name.insert_str(0, " ");
            }
            name.insert_str(0, format!("\x1b[{};{}H\x1b[1;31m", context.y, context.x).as_str());

            buffer.push_str(name.as_str());
            context.y += 1;
        }

        buffer.push_str(
            format!(
                "\x1b[{};{}H[{}/{}]",
                context.y + 1,
                context.x,
                self.hovered_item_index + 1,
                self.items.len()
            )
            .as_str(),
        );

        Render::new(buffer)
    }

    pub fn last_row(&mut self) -> usize {
        let mut last_row = self.maximum_row();
        if last_row > self.items.len() {
            last_row = self.items.len();
        }
        /*
        if let Some(item) = self.hovered_item() && item.expanded_index.is_some() {
            last_row -= DROPDOWN_ITEM_NAMES.len();
        }
        */

        last_row
    }

    fn maximum_row(&self) -> usize {
        self.first_row + height() - self.title_height() - 2
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
        let hovered_item = self.hovered_item().map(|i| i.clone());

        self.items.clear();
        let mut should_set_index = true;

        self.pane_type = if self.path.is_dir() {
            PaneType::Directory
        } else {
            PaneType::Preview
        };

        match self.pane_type {
            // read file and display lines as items
            PaneType::Preview => {
                if let Ok(slice) = read(&*self.path)
                    && let Ok(text) = String::from_utf8(slice)
                {
                    let mut lines = Vec::new();
                    let mut gutter = 0;
                    for (mut index, line) in text.split('\n').enumerate() {
                        index += 1;

                        let number = format!("{index}] ");
                        let number_len = number.len();
                        if number_len > gutter {
                            gutter = number_len;
                        }

                        lines.push(line);
                    }

                    for (mut index, line) in lines.iter().enumerate() {
                        index += 1;

                        let mut number = format!("{index}] ");
                        while number.len() < gutter {
                            number.insert(0, ' ');
                        }
                        let line = format!("{number}{line}");

                        self.items.push(Item::new(&line, false, true));
                        if line == hovered_name {
                            self.hovered_item_index = self.items.len() - 1;
                            should_set_index = false;
                        }
                    }
                }
            }
            PaneType::Directory => {
                let Ok(files) = read_dir(*self.path.clone()) else {
                    return;
                };

                for entry in files.flatten() {
                    let name = format_path(entry.path());

                    // check if it's the currently hovered Item
                    if let Some(hovered) = &hovered_item
                        && hovered.name() == name
                    {
                        self.items.push(hovered.clone());
                    } else {
                        self.items
                            .push(Item::new(&name, entry.path().is_dir(), false));
                    }

                    if name == hovered_name {
                        self.hovered_item_index = self.items.len() - 1;
                        should_set_index = false;
                    }
                }
            }
        }

        if should_set_index && !self.items.is_empty() && self.hovered_item_index >= self.items.len()
        {
            self.hovered_item_index = 0;
            self.first_row = 0;
        }
        let buffer = self.typing_buffer.is_some();

        if let Some(item) = self.hovered_item()
            && !buffer
        {
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
            if self.first_row + height() - self.title_height() - 1 <= self.hovered_item_index + 2 {
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
            let result = buffer.take_input(input)?;
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

    pub fn goto(&mut self, path: PathBuf) {
        self.pane_type = if path.is_dir() {
            PaneType::Directory
        } else {
            PaneType::Preview
        };
        *self.path = path;
        self.populate();
    }

    fn finish_buffer(&mut self, text: String) -> Option<UpdateItemResult> {
        let Some(buffer) = &mut self.typing_buffer else {
            unreachable!();
        };

        match buffer.location() {
            TypingLocation::Goto => {
                *self.path = PathBuf::from(text);
                self.typing_buffer = None;
                Some(UpdateItemResult::RepopulateAndHover)
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
                Some(UpdateItemResult::RepopulateAndHover)
            }
            TypingLocation::NewFile => {
                let mut path = self.path.clone();
                path.push(text);
                if let Err(e) = write(&*path, "") {
                    log(format!("pane::Pane::finish_buffer(): Couldn't create new file because {e}"));
                }
                self.typing_buffer = None;
                Some(UpdateItemResult::RepopulateAndHover)
            }
            TypingLocation::NewDirectory => {
                let mut path = self.path.clone();
                path.push(text);
                if let Err(e) = create_dir_all(&*path) {
                    log(format!("pane::Pane::finish_buffer(): Couldn't create new directory because {e}"));
                }
                self.typing_buffer = None;
                Some(UpdateItemResult::RepopulateAndHover)
            }
            _ => unreachable!(),
        }
    }

    fn title(&self, context: &RenderContext) -> String {
        let text = if let Some(buffer) = &self.typing_buffer && !matches!(buffer.location(), TypingLocation::NewDirectory | TypingLocation::NewFile) {
            if buffer.location() == TypingLocation::Goto {
                format!("Go to: \x1b[1;31m{}", buffer.display_buffer())
            } else {
                format!(
                    "{}\n\nFind: \x1b[1;31m{}\x1b[0m",
                    self.display_path(),
                    buffer.display_buffer()
                )
            }
        } else {
            let path = self.display_path();
            if let Some(pattern) = &self.search_pattern {
                format!("{}\n\nFind: \x1b[1;31m{}\x1b[0m", path, pattern)
            } else {
                path
            }
        };

        if context.x > 1 {
            text.replace('\n', format!("\n\x1b[{}C", context.x - 1).as_str())
        } else {
            text
        }
    }

    fn title_height(&self) -> usize {
        self.title(&RenderContext::new())
            .split('\n')
            .collect::<Vec<_>>()
            .len()
            + 1
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
    
    fn is_creating_file_or_directory(&self) -> bool {
        if let Some(buffer) = &self.typing_buffer && matches!(buffer.location(), TypingLocation::NewDirectory | TypingLocation::NewFile) {
            true
        } else {
            false
        }
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
            if let Err(e) = sender.send(Item::new(name, file.path().is_dir(), false)) {
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
        .last()
        .unwrap_or("[couldn't decode name]")
        .to_string()
}
