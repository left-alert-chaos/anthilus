use tuin::*;

fn main() {
    eprint!("\x1b[H\x1b[2J\x1b[3J");
    let column: Widget<Message> = col![
        Widget::Label(String::from("hello")),
        Widget::Label(String::from("Line 2"))
    ];
    let context = Default::default();
    let rendered: String = column.render(context).into();
    println!("{rendered}");
}

#[derive(Clone, Debug)]
enum Message {}
