use std::collections::hash_map::RandomState;
use std::fmt::{self, Display};
use std::hash::{BuildHasher, Hasher};
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

use serde::de::Visitor;
use thiserror::Error;

// #[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
            'K' => Ok(VerificationDigit::K),
            _ => Err(Error::VerificationDigitOutOfBounds(value.to_string())),
        }
    }
}

impl Into<char> for VerificationDigit {
    fn into(self) -> char {
        match self {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rut(Num, VerificationDigit);

impl Rut {
    /// Generates a random [`Rut`] instance.
    pub fn random() -> Self {
        let hasher = RandomState::new().build_hasher();
        let num = hasher.finish() as u32 % MAX_NUM;
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
        write!(f, "{}-{}", self.num(), self.vd())
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

        Err(Error::InvalidVerificationDigit { have: input_vd, want: want.vd().into() })
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

// #[cfg(feature = "serde")]
impl Serialize for Rut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.format(Format::Sans))
    }
}

struct RutVisitor;

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

impl<'de> Deserialize<'de> for Rut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RutVisitor)
    }
}

#[cfg(test)]
mod tests {
    use csv::ReaderBuilder;
    use serde::de::IntoDeserializer;
    use serde::de::value::{Error as ValueError, StrDeserializer, StringDeserializer};
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};

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
            (super::MIN_NUM, VerificationDigit::Nine),
            (super::MAX_NUM, VerificationDigit::Nine),
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

    #[test]
    fn parses_min_rut() {
        let string = MIN.to_string();
        assert_eq!(Rut::from_str(&string).unwrap(), MIN);
    }

    #[test]
    fn parses_max_rut() {
        let string = MAX.to_string();
        assert_eq!(Rut::from_str(&string).unwrap(), MAX);
    }

    #[test]
    fn format_sans_rut_value() {
        let have = "17.951.585-7";
        let want = "179515857";
        let rut = Rut::from_str(have).unwrap();

        assert_eq!(rut.format(Format::Sans), want);
    }

    #[test]
    fn format_dash_rut_value() {
        let have = "17.951.585-7";
        let want = "17951585-7";
        let rut = Rut::from_str(have).unwrap();

        assert_eq!(rut.format(Format::Dash), want);
    }

    #[test]
    fn format_dots_rut_value() {
        let cases = vec![
            ("179515857", "17.951.585-7"),
            ("75.303.649-0", "75.303.649-0"),
            ("273880941", "27.388.094-1"),
            ("27962409-2", "27.962.409-2"),
            ("98127523-3", "98.127.523-3"),
            ("30.686.957-4", "30.686.957-4"),
            ("450222755", "45.022.275-5"),
            ("615706396", "61.570.639-6"),
            ("59.608.778-7", "59.608.778-7"),
            ("43496204-8", "43.496.204-8"),
            ("700593819", "70.059.381-9"),
            ("92635843K", "92.635.843-K"),
        ];

        for (have, want) in cases {
            let rut = Rut::from_str(have).unwrap();
            assert_eq!(rut.format(Format::Dots), want);
        }
    }

    #[test]
    fn format_dots_rut_min() {
        let rut = MIN;
        assert_eq!(rut.format(Format::Dots), "1.000.000-9");
    }

    #[test]
    fn format_dots_rut_max() {
        let rut = MAX;
        assert_eq!(rut.format(Format::Dots), "99.999.999-9");
    }

    #[test]
    fn serialize_rut_instance() {
        let rut = Rut::from_str("92.635.843-K").unwrap();

        assert_tokens(&rut, &[Token::Str("92635843K")]);
    }

    #[test]
    fn deserialize_rut_as_str() {
        let rut: StrDeserializer<ValueError> = "450222755".into_deserializer();
        let rut = rut.deserialize_str(RutVisitor);

        assert_eq!(
            rut,
            Ok(Rut(45022275, VerificationDigit::Five))
        );
    }

    #[test]
    fn deserialize_rut_as_string() {
        let rut: StringDeserializer<ValueError> = String::from("450222755").into_deserializer();
        let rut = rut.deserialize_string(RutVisitor);

        assert_eq!(
            rut,
            Ok(Rut(45022275, VerificationDigit::Five))
        );
    }

    #[test]
    fn deserialize_rut_as_err_invalid_str() {
        assert_de_tokens_error::<Rut>(
            &[Token::Str("ThisIsNotARut")],
            "Provided string is not a number. invalid digit found in string",
        )
    }

    #[test]
    fn deserialize_rut_as_err_empty() {
        assert_de_tokens_error::<Rut>(
            &[Token::Str("")],
            "The provided string is empty",
        )
    }

    #[test]
    fn deserialize_rut_as_err() {
        assert_de_tokens_error::<Rut>(
            &[Token::Str("1.111.111-1")],
            "Invalid verification digit: have 1, want 4",
        )
    }
}
