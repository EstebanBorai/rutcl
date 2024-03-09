use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::hash::{BuildHasher, Hasher};
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

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

/// RUT's Number without the [`VerificationDigit`]
pub type Num = u32;

/// Max value for a RUT
const MAX: u32 = 99_999_999;

/// Min value for a RUT
const MIN: u32 = 1_000_000;

/// RUT value range
const RANGE: Range<u32> = MIN..MAX;

/// Product factor for RUT's Verification Digit Calculation
const FACTOR: [u32; 6] = [2, 3, 4, 5, 6, 7];

/// The total number of symbols in a RUT is used as constant on multiple
/// Verification Digit calculations
const SYMBOLS: u32 = 11;

/// Chilean RUT's Verification Digit
///
/// Refer: https://es.wikipedia.org/wiki/Rol_Único_Tributario
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VerificationDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    /// `K` Represent the Verification Digit `K`, which is equivalent to `10`
    K,
}

impl VerificationDigit {
    /// Creates a [`VerificationDigit`] from a RUT's body.
    ///
    /// # Theory
    ///
    /// In order to calculate the Verification Digit, the RUT's body is written
    /// backwards and each digit is multiplied by a factor. The factors are
    /// `[2, 3, 4, 5, 6, 7]`, the result is then summed.
    ///
    /// The sum of these digits is then divided by 11 and the remainder is
    /// then multiplied by 11.
    ///
    /// The result is the Verification Digit.
    pub fn new(num: Num) -> Result<Self, Error> {
        let mut digits = num
            .to_string()
            .chars()
            .rev()
            .map(|c| c.to_digit(10).expect("This code is unrachable"))
            .collect::<Vec<u32>>();
        let mut factor: usize = 0;
        let mut sum = 0;

        // Pop each digit from the backwards representation of RUT's body
        // and multiply it by the corresponding factor
        for digit in digits.iter_mut() {
            sum += *digit * (FACTOR[factor]);
            factor = (factor + 1) % 6;
        }

        // let remaining = (sum % SYMBOLS) as u32;
        let whole = sum / SYMBOLS;
        let base = sum - (SYMBOLS * whole);
        let digit = SYMBOLS - base;

        Self::from_u32(digit)
    }

    pub fn from_u32(value: u32) -> Result<Self, Error> {
        match value {
            1 => Ok(VerificationDigit::One),
            2 => Ok(VerificationDigit::Two),
            3 => Ok(VerificationDigit::Three),
            4 => Ok(VerificationDigit::Four),
            5 => Ok(VerificationDigit::Five),
            6 => Ok(VerificationDigit::Six),
            7 => Ok(VerificationDigit::Seven),
            8 => Ok(VerificationDigit::Eight),
            9 => Ok(VerificationDigit::Nine),
            10 => Ok(VerificationDigit::K),
            11 => Ok(VerificationDigit::Zero),
            _ => Err(Error::VerificationDigitOutOfBounds(value.to_string())),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            VerificationDigit::Zero => 0,
            VerificationDigit::One => 1,
            VerificationDigit::Two => 2,
            VerificationDigit::Three => 3,
            VerificationDigit::Four => 4,
            VerificationDigit::Five => 5,
            VerificationDigit::Six => 6,
            VerificationDigit::Seven => 7,
            VerificationDigit::Eight => 8,
            VerificationDigit::Nine => 9,
            VerificationDigit::K => 10,
        }
    }
}

impl FromStr for VerificationDigit {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "0" => Ok(VerificationDigit::Zero),
            "1" => Ok(VerificationDigit::One),
            "2" => Ok(VerificationDigit::Two),
            "3" => Ok(VerificationDigit::Three),
            "4" => Ok(VerificationDigit::Four),
            "5" => Ok(VerificationDigit::Five),
            "6" => Ok(VerificationDigit::Six),
            "7" => Ok(VerificationDigit::Seven),
            "8" => Ok(VerificationDigit::Eight),
            "9" => Ok(VerificationDigit::Nine),
            "K" => Ok(VerificationDigit::K),
            _ => Err(Error::VerificationDigitOutOfBounds(input.to_string())),
        }
    }
}

impl Display for VerificationDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            VerificationDigit::Zero => "0",
            VerificationDigit::One => "1",
            VerificationDigit::Two => "2",
            VerificationDigit::Three => "3",
            VerificationDigit::Four => "4",
            VerificationDigit::Five => "5",
            VerificationDigit::Six => "6",
            VerificationDigit::Seven => "7",
            VerificationDigit::Eight => "8",
            VerificationDigit::Nine => "9",
            VerificationDigit::K => "K",
        };

        write!(f, "{value}")
    }
}

/// Format for RUT's string representation
///
/// - `Sans`: RUT without any special characters
/// - `Dash`: RUT with a dash between the number and the verification digit
#[derive(Copy, Clone, Debug)]
pub enum Format {
    Sans,
    Dash,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rut(Num, VerificationDigit);

impl Rut {
    /// Generates a random [`Rut`] instance.
    pub fn random() -> Self {
        let hasher = RandomState::new().build_hasher();
        let num = hasher.finish() as u32 % MAX;
        let vd = VerificationDigit::new(num).unwrap();

        Rut(num, vd)
    }

    /// Return the RUT's number ([`Num`]) without the [`VerificationDigit`]
    #[inline]
    pub fn num(&self) -> Num {
        self.0
    }

    /// Return the DV output
    #[inline]
    pub fn vd(&self) -> VerificationDigit {
        self.1
    }

    pub fn format(&self, fmt: Format) -> String {
        match fmt {
            Format::Sans => format!("{}{}", self.0, self.1),
            Format::Dash => format!("{}-{}", self.0, self.1),
        }
    }

    /// Retrieves a "sans" RUT version.
    ///
    /// # Example
    ///
    /// ```
    /// use rutcl::Rut;
    ///
    /// let rut = Rut::sans("17.951.585-7");
    ///
    /// assert_eq!(rut, "179515857");
    /// ```
    pub fn sans<S: AsRef<str>>(input: S) -> String {
        input.as_ref().replace(['.', '-'], "")
    }
}

impl Display for Rut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.num(), self.vd())
    }
}

impl FromStr for Rut {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let sans = Rut::sans(input);

        if sans.is_empty() {
            return Err(Error::EmptyString);
        }

        let mut chars = sans.chars().collect::<Vec<char>>();

        // Discards the last char, assuming it is the verification digit
        chars.pop().expect(
            "Cannot extract verification digit from sans representation. Empty string found.",
        );

        let num = chars
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("")
            .parse::<Num>()
            .map_err(Error::NaN)?;

        Rut::try_from(num)
    }
}

impl TryFrom<Num> for Rut {
    type Error = Error;

    fn try_from(num: Num) -> Result<Self, Self::Error> {
        if RANGE.contains(&num) {
            let vd = VerificationDigit::new(num)?;
            Ok(Rut(num, vd))
        } else {
            Err(Error::OutOfRange)
        }
    }
}

#[cfg(test)]
mod tests {
    use csv::ReaderBuilder;

    use super::*;

    const SAMPLES: &str = include_str!("../fixtures/samples.csv");

    struct Sample {
        rut: String,
        num: String,
        vd: String,
    }

    fn samples() -> Vec<Sample> {
        let mut reader = ReaderBuilder::new().from_reader(SAMPLES.as_bytes());

        reader
            .records()
            .map(|record| {
                let record = record.unwrap();
                Sample {
                    rut: record[0].to_string(),
                    num: record[1].to_string(),
                    vd: record[2].to_string(),
                }
            })
            .collect::<Vec<Sample>>()
    }

    #[test]
    fn calculates_verification_digit() {
        let units = vec![
            (75_303_649, VerificationDigit::Zero),
            (27_388_094, VerificationDigit::One),
            (27_962_409, VerificationDigit::Two),
            (98_127_523, VerificationDigit::Three),
            (30_686_957, VerificationDigit::Four),
            (45_022_275, VerificationDigit::Five),
            (61_570_639, VerificationDigit::Six),
            (59_608_778, VerificationDigit::Seven),
            (43_496_204, VerificationDigit::Eight),
            (70_059_381, VerificationDigit::Nine),
            (92_635_843, VerificationDigit::K),
        ];

        for (number, expected) in units {
            let vd = VerificationDigit::new(number).unwrap();
            assert_eq!(vd, expected, "Expected: {:?}, Got: {:?}", expected, vd);
        }
    }

    #[test]
    fn parses_rut_from_string() {
        let samples = samples();

        samples.iter().for_each(|Sample { rut, num, vd }| {
            let rut = Rut::from_str(rut).unwrap();
            assert_eq!(rut.num(), num.parse::<Num>().unwrap());
            assert_eq!(rut.vd(), VerificationDigit::from_str(vd).unwrap());
            assert_eq!(rut.to_string(), format!("{}-{}", num, vd));
        });
    }

    #[test]
    fn random_never_repeats() {
        let mut ruts = vec![];

        for _ in 0..100 {
            let rut = Rut::random();
            assert!(!ruts.contains(&rut));
            ruts.push(rut);
        }
    }
}
