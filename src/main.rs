mod state;
mod ui;
mod logging;
mod username;
mod error;
use error::*;

use tuin::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::log("starting")?;

    if std::env::args().collect::<Vec<_>>().contains(&String::from("--log")) {
        logging::print_logs()
    } else {
        let mut state = state::Anthilus::new();
        state.event_loop(Some(state::check_reload));
        Ok(())
    }
}
