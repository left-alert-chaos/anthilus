mod logging;
mod render;
mod size;
mod state;
mod ui;
mod username;

use std::{env::set_current_dir, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::log("starting");

    for arg in std::env::args() {
        if arg.as_str() == "--log" {
            logging::print_logs();
            return Ok(());
        } else if arg.as_str() =="--help" {
            help();
            return Ok(());
        }

        let path = PathBuf::from(arg);
        if path.is_dir() {
            set_current_dir(path)?;
        }
    }


    let mut state = state::Anthilus::new();
    state.event_loop();
    Ok(())
}

fn help() {
    println!("Anthilus

Anthilus is a lightweight file manager for your Linux terminal!

Usage: anthilus [dir] [--log] [--help]
Flags:
    --log: Print logs and exit. Logs are located at ~/.cache/anthilus/anthilus.log
    --help: Print this help text and exit.

While using Anthilus, you can hit ? to see more help text.")
}
