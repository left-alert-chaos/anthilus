//! # state
//! This module holds logic for holding an app's state.

use crate::*;

use std::io::{self, Write};

/// # State
/// All behavior needed to store an app's persistent state.
/// Everything has a default implementation, so it can be implemented with no work.
pub trait State<T>
where T: Event
{
    /// Called once before event loop starts. It's optional and the default behavior does nothing.
    fn init(&mut self) {}

    /// Build the interface and return a root widget.
    fn build(&self) -> Option<Widget<T>> {
        Some(Widget::Label(String::from("Hello, world!")))
    }

    /// Update the state when an event is generated.
    fn update(&mut self, _event: T) {}

    /// Repeatedly run the event loop until `build()` returns `None`.
    /// The callback is called every loop iteration and if it returns `Some`, that message is passed
    /// to `update()`.
    fn event_loop(&mut self, callback: Option<fn() -> Option<T>>) {
        raw_mode();

        loop {
            // clear the screen
            print!("\x1b[H\x1b[2J\x1b[3J");
            
            // build, render and draw the UI
            let context = RenderContext::default();
            let rendered: String = match self.build() {
                Some(widget) => widget.render(context).into(),
                None => break,
            };
            print!("{rendered}");
            let _ = io::stdout().flush();

            if let Some(function) = &callback && let Some(message) = function() {
                self.update(message);
            }

            let input = match Input::get() {
                Ok(event) => event,
                Err(_) => continue,
            };

            let event = T::from(input);
            self.update(event);
        }

        exit_raw_mode();
    }
}
