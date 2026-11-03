use tuin::*;

use crate::logging::log;
use crate::ui::*;

use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Input(Input),
}

impl From<Input> for Message {
    fn from(value: Input) -> Self {
        Self::Input(value)
    }
}

impl Event for Message {}

#[derive(Debug, Clone)]
pub struct Anthilus {
    panes: Vec<Pane>,
    hovered_pane: usize,
    quit: bool,
    cursor_state: CursorState,
}

impl State<Message> for Anthilus {
    fn build(&self) -> Option<Widget> {
        if self.quit {
            return None;
        }

        let mut root = row![];
        for pane in &self.panes {
            root.add_child(pane.build());
        }
        Some(root)
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::Input(input) => self.update_input(input),
        }
    }
}

impl Anthilus {
    pub fn new() -> Self {
        Self {
            panes: vec![Pane::new(".")],
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
                    self.panes[self.hovered_pane].hovered = true;
                } else if self.panes.is_empty() {
                    self.quit = true;
                }
            }
            Input::Character('r') => self.get_hovered_pane().populate(),
            Input::Return => self.select(),
            Input::Character('p') => self.new_pane(),
            Input::Backspace => self.up(),
            _ => log(format!("recieved unused input {input:?}")),
        }
    }

    fn up(&mut self) {
        if *self.get_hovered_pane().path != PathBuf::from(".") {
            self.get_hovered_pane().path.pop();
        } else {
            if let Ok(path) = std::path::absolute(*self.get_hovered_pane().path.clone()) {
                *self.get_hovered_pane().path = path;
            }
        }
        self.get_hovered_pane().populate();
    }

    fn new_pane(&mut self) {
        let path = self.get_hovered_pane().hovered_item_path();

        if !path.is_dir() { return; }

        self.panes.push(
            Pane::new(path)
        );

        self.get_hovered_pane().hovered = false;
        self.hovered_pane = self.panes.len() - 1;
    }

    fn select(&mut self) {
        let path = self.get_hovered_pane().hovered_item_path();
        if path.is_dir() {
            *self.get_hovered_pane().path = path;
            self.get_hovered_pane().populate();
        } else if let Some(item) = self.get_hovered_pane().hovered_item() && let Some(result) = item.select() {
            self.use_update_result(result);
        }
    }

    fn update_input_typing(&mut self, input: Input) {
        if let Some(result) = self.get_hovered_pane().hovered_item().unwrap().input_typing(input) {
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
            UpdateItemResult::DoBoth(state) => {
                self.get_hovered_pane().populate();
                self.cursor_state = state;
            }
        }
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
    DoBoth(CursorState),
}
