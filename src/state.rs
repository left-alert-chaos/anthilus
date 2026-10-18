use tuin::*;

use crate::ui::*;

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
}
