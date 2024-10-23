#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::hash::{BuildHasher, Hasher};
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[cfg(feature = "serde")]
use std::fmt;

use rand::distributions::uniform::SampleRange;
use rand::{thread_rng, Rng};
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
use serde::de::Visitor;

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

/// Max number for a RUT without the Verification Digit
const MAX_NUM: u32 = 99_999_999;

/// Min number for a RUT without the Verification Digit
const MIN_NUM: u32 = 1_000_000;

/// Min value for a RUT
pub const MIN: Rut = Rut(MIN_NUM, VerificationDigit::Nine);

/// Max value for a RUT
pub const MAX: Rut = Rut(MAX_NUM, VerificationDigit::Nine);

/// RUT value range
const RANGE: RangeInclusive<u32> = MIN_NUM..=MAX_NUM;

/// Product factor for RUT's Verification Digit Calculation
const FACTOR: [u32; 6] = [2, 3, 4, 5, 6, 7];

/// The total number of symbols in a RUT is used as constant on multiple
/// Verification Digit calculations
const SYMBOLS: u32 = 11;

/// Chilean RUT's Verification Digit
///
/// Refer: https://es.wikipedia.org/wiki/Rol_Único_Tributario
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
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

impl Ord for VerificationDigit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_u32().cmp(&other.to_u32())
    }
}

impl PartialOrd for VerificationDigit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

impl TryFrom<char> for VerificationDigit {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(VerificationDigit::Zero),
            '1' => Ok(VerificationDigit::One),
            '2' => Ok(VerificationDigit::Two),
            '3' => Ok(VerificationDigit::Three),
            '4' => Ok(VerificationDigit::Four),
            '5' => Ok(VerificationDigit::Five),
            '6' => Ok(VerificationDigit::Six),
            '7' => Ok(VerificationDigit::Seven),
            '8' => Ok(VerificationDigit::Eight),
            '9' => Ok(VerificationDigit::Nine),
            'K' | 'k' => Ok(VerificationDigit::K),
            _ => Err(Error::VerificationDigitOutOfBounds(value.to_string())),
        }
    }
}

impl From<VerificationDigit> for char {
    fn from(val: VerificationDigit) -> Self {
        match val {
            VerificationDigit::Zero => '0',
            VerificationDigit::One => '1',
            VerificationDigit::Two => '2',
            VerificationDigit::Three => '3',
            VerificationDigit::Four => '4',
            VerificationDigit::Five => '5',
            VerificationDigit::Six => '6',
            VerificationDigit::Seven => '7',
            VerificationDigit::Eight => '8',
            VerificationDigit::Nine => '9',
            VerificationDigit::K => 'K',
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
#[derive(Copy, Clone, Debug)]
pub enum Format {
    /// No special characters. the RUT is formatted as a continuous set of
    /// digits followed by the verification digit without dash or dots.
    Sans,
    /// The RUT is formatted with a dash between the number and the
    /// verification digit. No dots are included.
    Dash,
    /// Fully qualified RUT notation, following the format `XX.XXX.XXX-X` which
    /// is printed in the Chilean ID cards.
    Dots,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Rut(Num, VerificationDigit);

impl Rut {
    /// Retrieves the maximum supported [`Rut`].
    ///
    /// Equivalent to using `rutcl::MAX`
    #[inline]
    pub const fn max() -> Self {
        MAX
    }

    /// Retieves the minimum supported [`Rut`].
    ///
    /// Equivalent to using `rutcl::MIN`
    #[inline]
    pub const fn min() -> Self {
        MIN
    }

    /// Generates a random [`Rut`] instance.
    pub fn random() -> Result<Self, Error> {
        let hasher = RandomState::new().build_hasher();
        let num = hasher.finish() as u32 % MAX_NUM;
        let vd = VerificationDigit::new(num)?;

        Ok(Rut(num, vd))
    }

    /// Generates a random [`Rut`] instance inside the provided range.
    pub fn random_in_range<R: SampleRange<u32>>(range: R) -> Result<Self, Error> {
        let num = thread_rng().gen_range(range);
        let vd = VerificationDigit::new(num)?;

        Ok(Rut(num, vd))
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
            Format::Dots => {
                let num = self.0.to_string();
                let mut chars = num.chars().collect::<Vec<char>>();
                let mut result = String::new();

                while !chars.is_empty() {
                    let chunk = chars.split_off(chars.len().saturating_sub(3));
                    let digits = chunk.into_iter().collect::<String>();

                    if result.is_empty() {
                        result = digits;
                    } else {
                        result = format!("{}.{}", digits, result);
                    }
                }

                format!("{}-{}", result, self.1)
            }
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
        let sans = self.format(Format::Sans);
        write!(f, "{sans}")
    }
}

impl FromStr for Rut {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let sans = Rut::sans(input);

        let mut chars = sans.chars().collect::<Vec<char>>();

        // Discards the last char, assuming it is the verification digit
        let Some(input_vd) = chars.pop() else {
            return Err(Error::EmptyString);
        };

        let num = chars
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("")
            .parse::<Num>()
            .map_err(Error::NaN)?;

        let want = Rut::try_from(num)?;

        if want.vd() == VerificationDigit::try_from(input_vd)? {
            return Ok(want);
        }

        Err(Error::InvalidVerificationDigit {
            have: input_vd,
            want: want.vd().into(),
        })
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

impl Ord for Rut {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 > other.0 {
            return Ordering::Greater;
        }

        if other.0 > self.0 {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Rut {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "serde")]
impl Serialize for Rut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.format(Format::Sans))
    }
}

#[cfg(feature = "serde")]
struct RutVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for RutVisitor {
    type Value = Rut;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Rut String instance formatted using the Sans format")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Rut::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Rut::from_str(v.as_str()).map_err(|err| E::custom(err.to_string()))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Rut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RutVisitor)
    }
}
