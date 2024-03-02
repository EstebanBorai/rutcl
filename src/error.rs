use std::num::ParseIntError;

use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("Invalid verification digit: have {have}, want {want}")]
    InvalidVerificationDigit { have: char, want: char },
    #[error("Verification digit out of bounds found: {0}")]
    VerificationDigitOutOfBounds(String),
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Provided string is not a number. {0}")]
    NaN(ParseIntError),
    #[error("Out of range")]
    OutOfRange,
    #[error("The provided string is empty")]
    EmptyString,
}
