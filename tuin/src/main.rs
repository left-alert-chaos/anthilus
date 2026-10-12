use tuin::*;

fn main() {
    let mut state = MyApp::new();
    state.event_loop(None);
}

#[derive(Clone, Debug)]
enum Message {
    Input(Input),
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
        let Message::Input(input) = event;
        self.letters.push(input.into());
    }

    fn build(&self) -> Option<Widget<Message>> {
        Some(col![
            Widget::Label(self.letters.clone()),
            Widget::Label(format!("It's been {} seconds.", self.seconds))
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
