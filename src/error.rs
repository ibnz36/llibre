use owo_colors::OwoColorize;
use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Inquire(inquire::InquireError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<inquire::InquireError> for Error {
    fn from(value: inquire::InquireError) -> Self {
        Self::Inquire(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_type = match self {
            Self::IO(_) => "io",
            Self::Inquire(_) => "cli",
        };

        write!(f, "{} {}", error_type.red().bold(), "error:".red().bold())?;

        match self {
            Self::IO(e) => writeln!(f, "{}", e),
            Self::Inquire(e) => writeln!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::IO(e) => e,
            Self::Inquire(e) => e,
        })
    }
}
