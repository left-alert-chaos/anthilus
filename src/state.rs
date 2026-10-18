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
    panes: Vec<Pane>
}

impl State<Message> for Anthilus {}
