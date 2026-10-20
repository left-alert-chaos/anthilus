use tuin::*;

#[derive(Clone, Debug)]
pub struct Item {
    name: String,
    expanded: bool,
    pub hovered: bool,
}

impl Item {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            expanded: false,
            hovered: false,
        }
    }

    pub fn build(&self) -> Widget {
        let mut text = self.name.clone();

        // invert colors if hovered
        if self.hovered {
            text.insert_str(0, "\x1b[1;31m");
            text.push_str("\x1b[0m");
        }

        Widget::Label(text)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
