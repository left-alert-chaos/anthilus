pub mod input;
pub mod runtime;

use input::Input;

use crate::logging::log;
use crate::render::RenderContext;
use crate::ui::*;

use std::{env::current_dir, path::PathBuf, sync::LazyLock};

static CURRENTDIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("."));

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Input(Input),
    Repopulate,
}

impl From<Input> for Message {
    fn from(value: Input) -> Self {
        Self::Input(value)
    }
}

#[derive(Debug)]
pub struct Anthilus {
    panes: Vec<Pane>,
    hovered_pane: usize,
    quit: bool,
    cursor_state: CursorState,
}

impl Anthilus {
    pub fn render(&self) -> Option<String> {
        if self.quit {
            return None;
        }

        let mut context = RenderContext::new();
        context.width /= self.panes.len() as u32;

        let mut buffer = String::new();
        for pane in &self.panes {
            let render = pane.render(&mut context);
            buffer = format!("{buffer}{}", render.text);
            context.x += render.width as u32;
            context.y = 1;
        }
        Some(buffer)
    }

    pub fn update(&mut self, event: Message) {
        match event {
            Message::Input(input) => self.update_input(input),
            Message::Repopulate => {
                for pane in self.panes.iter_mut() {
                    pane.populate();
                }
            }
        }
    }

    pub fn new() -> Self {
        Self {
            panes: vec![Pane::new(current_dir().unwrap())],
            hovered_pane: 0,
            quit: false,
            cursor_state: CursorState::Hovering,
        }
    }

    fn update_input(&mut self, input: Input) {
        match self.cursor_state {
            CursorState::Hovering => self.update_input_hovering(input),
            CursorState::Typing => self.update_input_typing(input),
        }
    }

    fn update_input_hovering(&mut self, input: Input) {
        match input {
            Input::Left | Input::Character('h') => {
                if self.hovered_pane > 0 {
                    self.get_hovered_pane().hovered = false;
                    self.hovered_pane -= 1;
                    self.get_hovered_pane().hovered = true;
                }
            }
            Input::Right | Input::Character('l') => {
                if self.hovered_pane < self.panes.len() - 1 {
                    self.get_hovered_pane().hovered = false;
                    self.hovered_pane += 1;
                    self.get_hovered_pane().hovered = true;
                }
            }
            Input::Up | Input::Character('k') => self.get_hovered_pane().move_highlight_up(),
            Input::Down | Input::Character('j') => self.get_hovered_pane().move_highlight_down(),
            Input::Character('q') => {
                self.panes.remove(self.hovered_pane);
                if self.hovered_pane > 0 {
                    self.hovered_pane -= 1;
                }
                if self.panes.is_empty() {
                    self.quit = true;
                } else {
                    self.panes[self.hovered_pane].hovered = true;
                }
            }
            Input::Character('r') => self.get_hovered_pane().populate(),
            Input::Return => self.select(),
            Input::Character('n') => self.new_file(),
            Input::Character('p') => self.new_pane(),
            Input::Backspace => self.up(),
            Input::Character('g') => self.goto(),
            Input::Character('f') => self.search(),
            Input::Escape => {
                self.get_hovered_pane().stop_searching();
                self.get_hovered_pane().populate();
            }
            _ => log(format!("recieved unused input {input:?}")),
        }
    }

    fn up(&mut self) {
        if *self.get_hovered_pane().path != *CURRENTDIR {
            self.get_hovered_pane().stop_searching();
            self.get_hovered_pane().path.pop();
        } else {
            if let Ok(path) = std::path::absolute(".") {
                *self.get_hovered_pane().path = path;
            }
        }
        self.get_hovered_pane().populate();
    }

    fn new_pane(&mut self) {
        let path = self.get_hovered_pane().hovered_item_path();

        self.panes.push(Pane::new(path));

        self.get_hovered_pane().hovered = false;
        self.hovered_pane = self.panes.len() - 1;
    }

    fn select(&mut self) {
        let pane = self.get_hovered_pane();
        let path = pane.hovered_item_path();
        if let Some(item) = pane.hovered_item()
            && item.is_preview_line
        {
            return;
        } else if pane.hovered_item().is_none() {
            return;
        }

        let item_index = pane.hovered_item_index;
        let last_row = pane.last_row();

        if let Some(item) = pane.hovered_item()
        {
            let res = item.select(path);
            let expanded_index = item.expanded_index;

            if let Some(index) = expanded_index && index + item_index + 2 > last_row {
                log("increasing first row");
                pane.first_row += DROPDOWN_ITEM_NAMES.len() - index - 1;
            }

            if let Some(result) = res {
                self.use_update_result(result);
            }
        }

    }

    fn update_input_typing(&mut self, input: Input) {
        if let Some(result) = self.get_hovered_pane().update_typing(input) {
            self.use_update_result(result);
        }
    }

    fn get_hovered_pane(&mut self) -> &mut Pane {
        &mut self.panes[self.hovered_pane]
    }

    fn use_update_result(&mut self, result: UpdateItemResult) {
        match result {
            UpdateItemResult::Repopulate => self.get_hovered_pane().populate(),
            UpdateItemResult::SetCursorState(state) => self.cursor_state = state,
            UpdateItemResult::RepopulateAndHover => {
                self.get_hovered_pane().populate();
                self.cursor_state = CursorState::Hovering;
            }
            UpdateItemResult::Goto => {
                let path = self.get_hovered_pane().hovered_item_path();
                self.get_hovered_pane().goto(path);
            }
            UpdateItemResult::NewPane => {
                let path = self.get_hovered_pane().hovered_item_path();
                self.get_hovered_pane().hovered = false;
                self.panes.push(Pane::new(path));
                self.hovered_pane = self.panes.len() - 1;
                self.get_hovered_pane().hovered = true;
            }
        }
    }

    fn new_file(&mut self) {
        let mut file_number = 0;
        let mut file_name = String::from("file0");

        // this has probably been properly styled by now but it was one hell of a one-liner when I
        // wrote it
        while !self
            .get_hovered_pane()
            .items
            .iter()
            .filter(|i| i.name() == file_name)
            .collect::<Vec<_>>()
            .is_empty()
        {
            file_number += 1;
            file_name = format!("file{file_number}");
        }

        let mut path = self.get_hovered_pane().path.clone();
        path.push(&file_name);
        if let Err(e) = std::fs::write(*path, "") {
            log(format!(
                "Anthilus::new_file(): Couldn't write to {file_name} because {e}"
            ));
        }

        self.get_hovered_pane().populate();
    }

    fn goto(&mut self) {
        let pane = self.get_hovered_pane();
        if let Some(item) = pane.hovered_item() {
            item.hovered = false;
        }

        pane.typing_buffer = Some(TypingBuffer::new(String::new(), TypingLocation::Goto));
        self.cursor_state = CursorState::Typing;
    }

    fn search(&mut self) {
        let pane = self.get_hovered_pane();
        if let Some(item) = pane.hovered_item() {
            item.hovered = false;
        }

        pane.typing_buffer = Some(TypingBuffer::new(String::new(), TypingLocation::Search));
        self.cursor_state = CursorState::Typing;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum CursorState {
    Typing,
    Hovering,
}

/// What do do when an Item finishes updating
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpdateItemResult {
    Repopulate,
    SetCursorState(CursorState),
    RepopulateAndHover,
    Goto,
    NewPane,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TypingLocation {
    Rename,
    Goto,
    Search,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypingBuffer {
    buffer: String,
    cursor: usize,
    location: TypingLocation,
}

impl TypingBuffer {
    pub fn new(buffer: String, location: TypingLocation) -> Self {
        Self {
            buffer,
            cursor: 0,
            location,
        }
    }

    /// Take some input and return whether to use the buffer, cancel operation, or wait
    pub fn take_input(&mut self, input: Input) -> Option<BufferInputResult> {
        match input {
            Input::Character(c) => self.buffer.push(c),
            Input::Escape => {
                log("Escaping buffer");
                return Some(BufferInputResult::Cancel);
            }
            Input::Return => return Some(BufferInputResult::Complete(self.buffer.clone())),
            Input::Backspace => {
                self.buffer.pop();
            }
            _ => {}
        }

        None
    }

    pub fn location(&self) -> TypingLocation {
        self.location
    }

    pub fn display_buffer(&self) -> String {
        format!("{}\x1b[7m \x1b[0m", self.buffer)
    }
}

/// What to do, if anything, after a `TypingBuffer` takes input
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BufferInputResult {
    Complete(String),
    Cancel,
}
