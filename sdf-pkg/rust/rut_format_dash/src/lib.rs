use std::str::FromStr;

use anyhow::Result;
use rutcl::{Rut, Format};

use sdf_macros::sdf;

#[sdf(map, package = "rut-format-dash", namespace = "estebanborai")]
pub(crate) fn rut_format_dash(input: String) -> Result<String, String> {
    Ok(Rut::from_str(&input).map_err(|err| err.to_string())?.format(Format::Dash))
}
