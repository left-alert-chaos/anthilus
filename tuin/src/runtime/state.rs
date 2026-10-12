//! # state
//! This module holds logic for holding an app's state.

use crate::*;

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

    /// Build, render, and print a UI
    fn draw(&self) {
        let context = RenderContext::default();
        let rendered: String = match self.build() {
            Some(widget) => widget.render(context).into(),
            None => return,
        };
        print!("{rendered}");
    }

    /// Repeatedly run the event loop until `build()` returns `None`.
    fn event_loop(&mut self) {
        
    }
}
