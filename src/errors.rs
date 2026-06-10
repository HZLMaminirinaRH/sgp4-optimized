use std::fmt;

#[derive(Debug, Clone)]
pub enum SGP4Error {
    InvalidTLE(String),
    ParseError(String),
    PropagationError(String),
}

impl fmt::Display for SGP4Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SGP4Error::InvalidTLE(msg) => write!(f, "Erreur TLE : {}", msg),
            SGP4Error::ParseError(msg) => write!(f, "Erreur de parsing TLE : {}", msg),
            SGP4Error::PropagationError(msg) => write!(f, "Erreur de propagation SGP4 : {}", msg),
        }
    }
}

impl std::error::Error for SGP4Error {}
