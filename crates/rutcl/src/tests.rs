use csv::ReaderBuilder;

#[cfg(feature = "serde")]
use serde::de::value::{Error as ValueError, StrDeserializer, StringDeserializer};
#[cfg(feature = "serde")]
use serde::de::IntoDeserializer;
#[cfg(feature = "serde")]
use serde_test::{assert_de_tokens_error, assert_tokens, Token};

use super::*;

const SAMPLES: &str = include_str!("../../../fixtures/samples.csv");

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
        assert_eq!(rut.to_string(), format!("{}{}", num, vd));
    });
}

#[test]
fn random_never_repeats() {
    let mut ruts = vec![];

    for _ in 0..100 {
        let rut = Rut::random().unwrap();
        assert!(!ruts.contains(&rut));
        ruts.push(rut);
    }
}

#[test]
fn associated_fn_max() {
    assert_eq!(Rut::max(), MAX);
}

#[test]
fn associated_fn_min() {
    assert_eq!(Rut::min(), MIN);
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
#[cfg(feature = "serde")]
fn serialize_rut_instance() {
    let rut = Rut::from_str("92.635.843-K").unwrap();

    assert_tokens(&rut, &[Token::Str("92635843K")]);
}

#[test]
#[cfg(feature = "serde")]
fn deserialize_rut_as_str() {
    let rut: StrDeserializer<ValueError> = "450222755".into_deserializer();
    let rut = rut.deserialize_str(RutVisitor);

    assert_eq!(rut, Ok(Rut(45022275, VerificationDigit::Five)));
}

#[test]
#[cfg(feature = "serde")]
fn deserialize_rut_as_string() {
    let rut: StringDeserializer<ValueError> = String::from("450222755").into_deserializer();
    let rut = rut.deserialize_string(RutVisitor);

    assert_eq!(rut, Ok(Rut(45022275, VerificationDigit::Five)));
}

#[test]
#[cfg(feature = "serde")]
fn deserialize_rut_as_err_invalid_str() {
    assert_de_tokens_error::<Rut>(
        &[Token::Str("ThisIsNotARut")],
        "Provided string is not a number. invalid digit found in string",
    )
}

#[test]
#[cfg(feature = "serde")]
fn deserialize_rut_as_err_empty() {
    assert_de_tokens_error::<Rut>(&[Token::Str("")], "The provided string is empty")
}

#[test]
#[cfg(feature = "serde")]
fn deserialize_rut_as_err() {
    assert_de_tokens_error::<Rut>(
        &[Token::Str("1.111.111-1")],
        "Invalid verification digit: have 1, want 4",
    )
}

#[test]
fn compares_ruts() {
    let ruts = vec![
        ("1326658-1", "15441715-K"),
        ("15441715-K", "29718958-1"),
        ("29718958-1", "30088687-6"),
        ("30088687-6", "45278657-5"),
        ("45278657-5", "58175019-6"),
        ("58175019-6", "63990386-9"),
        ("63990386-9", "77992926-4"),
        ("77992926-4", "80919766-2"),
        ("80919766-2", "94577430-4"),
    ];

    for (rut_a, rut_b) in ruts {
        assert!(
            Rut::from_str(rut_a).unwrap() < Rut::from_str(rut_b).unwrap(),
            "{rut_a} should be lower than {rut_b}"
        );
        assert!(
            Rut::from_str(rut_b).unwrap() > Rut::from_str(rut_a).unwrap(),
            "{rut_b} should be greather than {rut_a}"
        );
    }
}

#[test]
fn compares_equal_ruts() {
    let ruts = vec![
        ("15441715-K", "15441715-K", true),
        ("15441715-K", "29718958-1", false),
        ("30088687-6", "30088687-6", true),
        ("30088687-6", "45278657-5", false),
    ];

    for (rut_a, rut_b, expect) in ruts {
        assert_eq!(
            Rut::from_str(rut_a).unwrap() == Rut::from_str(rut_b).unwrap(),
            expect
        );
    }
}

#[test]
fn support_lowercase_k() {
    let rut = Rut::from_str("15441715-k").expect("Should build RUT instance");

    assert_eq!(rut.1, VerificationDigit::K);
}

#[test]
#[cfg(feature = "rand")]
fn generates_random_in_range() {
    let mut prevs = Vec::with_capacity(100);

    for i in 0..100 {
        let rut = Rut::random_in_range(10_000_000..15_000_000).unwrap();
        assert!(
            !prevs.contains(&rut),
            "RUT: {:?} (Number {}) was generated before within a max of 100 iterations, current iteration {}",
            rut,
            rut.0,
            i + 1,
        );
        prevs.push(rut);
        assert!(
            10_000_000 <= rut.0 && rut.0 <= 15_000_000,
            "RUT: {:?} (Number {}) outbounds range, current iteration {}",
            rut,
            rut.0,
            i + 1,
        );
    }
}
