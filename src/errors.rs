use std::{
    error::Error,
    fmt::{self},
};

#[derive(Debug, Clone)]
pub struct MFError(pub String);

impl fmt::Display for MFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for MFError {}
