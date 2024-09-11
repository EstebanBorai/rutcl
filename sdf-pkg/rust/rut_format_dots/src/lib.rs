use std::str::FromStr;

use anyhow::Result;
use rutcl::{Rut, Format};

use sdf_macros::sdf;

#[sdf(map, package = "rut-format-dots", namespace = "eborai")]
pub(crate) fn rut_format_dots(input: String) -> Result<String, String> {
    Ok(Rut::from_str(&input).map_err(|err| err.to_string())?.format(Format::Dots))
}
