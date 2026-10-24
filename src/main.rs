mod state;
mod ui;
mod logging;

use tuin::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::log("starting")?;
    let mut state = state::Anthilus::new();
    state.event_loop(Some(state::check_reload));
    Ok(())
}
