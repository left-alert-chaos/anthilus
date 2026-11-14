use super::Anthilus;
use crate::{logging::log, state::{input::{raw_mode, exit_raw_mode, set_stdin_nonblocking, Input}, Message}};
use std::{time::{Duration, Instant}, io::{stdout, Write}};

impl Anthilus {
    pub fn event_loop(&mut self) {
        log("Anthilus::event_loop(): Starting event loop");

        raw_mode();
        let mut info = RuntimeInfo::new();
        set_stdin_nonblocking();

        print!("\x1b[?25l\x1b[?1049h");

        loop {
            info.frame += 1;
            print!("\x1b[H\x1b[2J\x1b[3J");

            if let Some(text) = self.render() {
                print!("{text}");
            } else {
                break;
            }
            let _ = stdout().flush();

            let input = match loop {
                if let Ok(event) = Input::get() {
                    break Some(event);
                } else if let Some(event) = repopulate_check(&mut info)
                {
                    self.update(event);
                    break None;
                }
            } {
                Some(event) => event,
                None => continue,
            };

            let event = Message::Input(input);
            self.update(event);
        }

        print!("\x1b[?1049l\x1b[?25h");

        log("State<T>::event_loop(): Ending event loop");
        exit_raw_mode();
        }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeInfo {
    last_instant: Instant,
    frame: u64,
}

impl RuntimeInfo {
    fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            frame: 0,
        }
    }

    /// Get the elapsed time sinse the clock was reset
    pub fn elapsed(&self) -> Duration {
        self.last_instant.elapsed()
    }

    /// Reset the clock
    pub fn reset_clock(&mut self) {
        self.last_instant = Instant::now();
    }

    /// Get the numbered frame we're currently on. The first frame is 1.
    pub fn frame(&self) -> u64 {
        self.frame
    }
}

pub fn repopulate_check(info: &mut RuntimeInfo) -> Option<Message> {
    if info.elapsed().as_secs_f64() > 0.25 {
        info.reset_clock();
        Some(Message::Repopulate)
    } else {
        None
    }
}
