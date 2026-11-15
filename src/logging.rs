use std::fs::{read, write};

pub fn log(s: impl ToString) {
    let username = crate::username::username();
    let path = format!("/home/{username}/.cache/anthilus/anthilus.log");

    let mut text = String::from_utf8(read(&path).unwrap_or(Vec::new())).unwrap_or_default();
    text.push_str(&s.to_string());
    text.push('\n');

    let _ = write(&path, text);
}

pub fn print_logs() {
    let username = crate::username::username();
    let path = format!("/home/{username}/.cache/anthilus/anthilus.log");

    let text = match read(&path) {
        Ok(bytes) => String::from_utf8(bytes).unwrap_or_default(),
        Err(_) => {
            let _ = write(&path, "");
            String::new()
        }
    };

    println!("{text}");
}
