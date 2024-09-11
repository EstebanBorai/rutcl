use std::str::FromStr;

use anyhow::Result;
use rutcl::Rut;

use sdf_macros::sdf;

#[sdf(filter, package = "rut-is-valid", namespace = "estebanborai")]
pub(crate) fn rut_is_valid(input: String) -> Result<bool, String> {
    if Rut::from_str(&input).is_ok() {
        return Ok(true);
    }

    Ok(false)
}
