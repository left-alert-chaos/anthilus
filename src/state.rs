use tuin::*;

use crate::logging::log;
use crate::ui::*;

use std::{fs, time::Duration};

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Input(Input),
    Reload,
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
            Message::Reload => self.get_hovered_pane().populate(),
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
                    self.hovered_pane -= 1;
                }
            }
            Input::Right | Input::Character('l') => {
                if self.hovered_pane < self.panes.len() - 1 {
                    self.hovered_pane += 1;
                }
            }
            Input::Up | Input::Character('k') => self.get_hovered_pane().move_highlight_up(),
            Input::Down | Input::Character('j') => self.get_hovered_pane().move_highlight_down(),
            Input::Character('q') => self.quit = true,
            Input::Character('r') => self.get_hovered_pane().populate(),
            Input::Return => self.select(),
            _ => {}
        }
    }

    fn select(&mut self) {
        let path = self.get_hovered_pane().hovered_item_path();
        let _ = log(format!("path is {path:?}"));
        if path.is_dir() {
            self.get_hovered_pane().change_directory(path);
            let _ = log("changing dir");
        } else if let Some(item) = self.get_hovered_pane().hovered_item() {
            let _ = log("expanding");
            item.expand();
        }
    }

    fn update_input_typing(&mut self, _input: Input) {}

    fn get_hovered_pane(&mut self) -> &mut Pane {
        &mut self.panes[self.hovered_pane]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CursorState {
    Typing,
    Hovering,
}

/// Run as a callback every frame
pub fn check_reload(info: &mut RuntimeInfo) -> Option<Message> {
    // reload every 5 seconds
    if info.elapsed() >= Duration::new(5, 0) {
        info.reset_clock();
        Some(Message::Reload)
    } else {
        None
    }
}
