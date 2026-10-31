use tuin::*;

#[derive(Clone, Debug)]
pub struct Item {
    name: String,
    pub expanded_index: Option<usize>,
    pub hovered: bool,
}

impl Item {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            expanded_index: None,
            hovered: false,
        }
    }

    pub fn build(&self, pane_is_hovered: bool) -> Widget {
        let mut text = self.name.clone();

        // invert colors if hovered
        if self.hovered && pane_is_hovered {
            text.insert_str(0, "\x1b[1;31m ");
        }
        text.push_str("\x1b[0m");

        let mut main = Widget::Label(text);

        if let Some(index) = self.expanded_index {
            main = col![
                main,
                Widget::Label(String::from("  test")),
                Widget::Label(String::from("  another thing"))
            ];

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

    pub fn toggle_expand(&mut self) {
        if self.expanded_index.is_some() {
            self.expanded_index = None;
        } else {
            self.expanded_index = Some(0);
        }
    }
}
