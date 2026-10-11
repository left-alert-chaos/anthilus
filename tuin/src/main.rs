use tuin::*;

fn main() {
    // clear the screen
    eprint!("\x1b[H\x1b[2J\x1b[3J");

    // build the widgets
    let column: Widget<Message> = row![
        Widget::Label(String::from("hello")),
        Widget::Label(String::from("Line 2")),
        col![
            Widget::Label(String::from("item1")),
            Widget::Label(String::from("item2"))
        ],
        Widget::Label(String::from("I'm also a thing!"))
    ];

    // rendering
    let context = Default::default();
    let rendered: String = column.render(context).into();
    println!("{rendered}\n\n");
}

#[derive(Clone, Debug)]
enum Message {}
