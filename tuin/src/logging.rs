//! # logging
//! This file holds logging logic copied from Anthilus. All Tuin programs share the same log, which
//! is mostly used only by the renderer.

use std::{error::Error, fs};

pub fn program_log(s: impl ToString, program_name: &str) -> Result<(), Box<dyn Error>> {
    let user = crate::username();
    if !fs::exists(format!("/home/{user}/.cache/{program_name}/{program_name}.log")).unwrap_or(false) {
        println!("creating dir");
        fs::create_dir_all(format!("/home/{user}/.cache/{program_name}/")).unwrap();
        fs::write(
            format!("/home/{user}/.cache/{program_name}/{program_name}.log"),
            format!("created ~/.cache/{program_name} and ~/.cache/{program_name}.log"),
        )
        .unwrap();
    }

    let mut text =
        String::from_utf8(fs::read(format!("/home/{user}/.cache/{program_name}/{program_name}.log")).unwrap())
            .unwrap();
    text.push('\n');
    text.push_str(s.to_string().as_str());
    fs::write(format!("/home/{user}/.cache/{program_name}/{program_name}.log"), text).unwrap();

    Ok(())
}

pub fn print_program_logs(program_name: &str) -> Result<(), Box<dyn Error>> {
    let user = crate::username();
    println!(
        "{}",
        String::from_utf8(fs::read(format!(
            "/home/{user}/.cache/{program_name}/{program_name}.log"
        ))?)?
    );
    Ok(())
}

pub fn log(s: impl ToString) -> Result<(), Box<dyn Error>> {
    program_log(s, "tuin")
}
