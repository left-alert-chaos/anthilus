use std::error::Error;

use tuin::{print_program_logs, program_log};

pub fn log(s: impl ToString) -> Result<(), Box<dyn Error>> {
    program_log(s, "anthilus")
}

pub fn print_logs() -> Result<(), Box<dyn Error>> {
    print_program_logs("anthilus")
}
