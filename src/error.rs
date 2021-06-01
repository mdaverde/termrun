use std::{error, ffi, fmt};

#[derive(Debug)]
#[non_exhaustive]
pub enum CliError {
    StringConv,
    TooLongCmd,
    Unix(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::StringConv => write!(f, "Did not understand text format in args"),
            CliError::TooLongCmd => write!(f, "Too many commands passed in"),
            CliError::Unix(err) => err.fmt(f),
        }
    }
}

impl error::Error for CliError {}

impl From<ffi::NulError> for CliError {
    fn from(_: ffi::NulError) -> Self {
        CliError::StringConv
    }
}

impl From<nix::Error> for CliError {
    fn from(err: nix::Error) -> Self {
        CliError::Unix(err.to_string())
    }
}
