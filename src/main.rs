mod state;
mod ui;

use tuin::State;

fn main() {
    let mut state = state::Anthilus::new();
    state.event_loop(Some(state::check_reload));
}
