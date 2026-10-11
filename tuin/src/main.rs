use tuin::*;

fn main() {
    let column: Widget<Message> = col![
        Widget::Label(String::from("hello")),
        Widget::Label(String::from("Line 2"))
    ];
    let context = RenderContext {
        x: 0,
        y: 0,
        max_width: 100,
        terminal_width: 100,
    };
    let rendered: String = column.render(context).into();
    print!("{rendered}");
}

#[derive(Clone, Debug)]
enum Message {}
