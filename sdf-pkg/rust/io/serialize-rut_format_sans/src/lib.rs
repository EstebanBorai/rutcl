pub mod bindings;
mod serialize {
    use crate::bindings;
    struct Component;
    bindings::export!(Component with_types_in bindings);
    use crate::bindings::exports::estebanborai::serialize_rut_format_sans::serialize::Guest as SerializeOutputInterface;
    impl SerializeOutputInterface for Component {
        fn serialize_key(output: Option<Vec<u8>>) -> Result<Option<Vec<u8>>, String> {
            match serialize_key_impl(output) {
                Ok(out) => Ok(out),
                Err(err) => {
                    eprintln!("Error serializing key {err}");
                    Err(err)
                }
            }
        }
        fn serialize_output(output: String) -> Result<Vec<u8>, String> {
            match serialize_output_impl(output) {
                Ok(out) => Ok(out),
                Err(err) => {
                    eprintln!("Error serializing output {err}");
                    Err(err)
                }
            }
        }
    }
    fn serialize_key_impl(output: Option<Vec<u8>>) -> Result<Option<Vec<u8>>, String> {
        let Some(output) = output else {
            return Ok(None);
        };
        Ok(Some(output))
    }
    fn serialize_output_impl(output: String) -> Result<Vec<u8>, String> {
        serde_json::to_vec(&output).map_err(|err| err.to_string())
    }
}
