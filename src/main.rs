mod logging;
mod state;
mod ui;

use tuin::State;

use std::{path::PathBuf, env::set_current_dir};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::log("starting");

    let mut log = false;
    for arg in std::env::args() {
        if arg.as_str() == "--log" { log = true; continue; }
        
        let path = PathBuf::from(arg);
        if path.is_dir() {
            set_current_dir(path)?;
        }
    }

    if log {
        logging::print_logs()
    } else {
        let mut state = state::Anthilus::new();
        state.event_loop(Some(state::check_reload));
        Ok(())
    }
}
