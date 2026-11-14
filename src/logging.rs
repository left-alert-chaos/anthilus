use std::error::Error;

pub fn log(s: impl ToString) {
    let _ = program_log(s, "anthilus");
}

pub fn print_logs() -> Result<(), Box<dyn Error>> {
    print_program_logs("anthilus")
}
