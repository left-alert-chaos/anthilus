use std::{error::Error, fmt};

#[derive(Copy, Clone, Debug)]
pub enum AnthilusError {}

impl fmt::Display for AnthilusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "anthilus had an error")
    }
}

impl Error for AnthilusError {}
