use std::{error::Error, fmt};

#[derive(Copy, Clone, Debug)]
pub enum AnthilusError {
    NoUsername,
}

impl fmt::Display for AnthilusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::NoUsername => "couldn't determine username",
        };
        write!(f, "{text}")
    }
}

impl Error for AnthilusError {}
