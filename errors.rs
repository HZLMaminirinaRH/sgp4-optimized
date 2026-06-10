use thiserror::Error;

pub enum SGP4Error {
    InvalidTLE(String),
    PropagationFailed(String),
    DegenerateOrbit,
    TimeOutOfBounds,
}
