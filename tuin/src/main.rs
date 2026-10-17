use tuin::*;

use std::time::Duration;

fn main() {
    let mut state = MyApp::new();
    state.event_loop(Some(info_callback));
}

#[derive(Clone, Debug)]
enum Message {
    Input(Input),
    Tick,
}

impl Event for Message {}

impl From<Input> for Message {
    fn from(value: Input) -> Self {
        Self::Input(value)
    }
}

struct MyApp {
    letters: String,
    seconds: u64,
}

impl State<Message> for MyApp {
    fn update(&mut self, event: Message) {
        if let Message::Input(input) = event {
            self.letters.push(input.into());
        } else {
            self.seconds += 1;
        }
    }

    fn build(&self) -> Option<Widget> {
        Some(col![
            Widget::Label(self.letters.clone()),
            Widget::Label(format!("It's been {} seconds.", self.seconds)),
            Widget::Pad,
            row![
                Widget::Label(String::from("This is a\nmultiline label!")),
                Widget::Label(String::from("This is also a\nmultiline label!"))
            ]
        ])
    }
}

impl MyApp {
    fn new() -> Self {
        Self {
            letters: String::new(),
            seconds: 0,
        }
    }
}

fn info_callback(info: &mut RuntimeInfo) -> Option<Message> {
    if info.elapsed() >= Duration::new(1, 0) {
        info.reset_clock();
        Some(Message::Tick)
    } else {
        None
    }
}
