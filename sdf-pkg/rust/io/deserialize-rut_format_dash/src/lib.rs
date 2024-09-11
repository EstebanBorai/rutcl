pub mod bindings;
mod serialize {
    use crate::bindings;
    struct Component;
    bindings::export!(Component with_types_in bindings);
    use crate::bindings::exports::estebanborai::deserialize_rut_format_dash::deserialize::Guest as DeserializeInputInterface;
    impl DeserializeInputInterface for Component {
        fn deserialize_key(
            input_str: Option<String>,
        ) -> Result<Option<Vec<u8>>, String> {
            match deserialize_key_impl(input_str) {
                Ok(input) => Ok(input),
                Err(err) => {
                    eprintln!("Error deserializing input key {err}");
                    Err(err)
                }
            }
        }
        fn deserialize_input(input_str: String) -> Result<String, String> {
            match deserialize_input_impl(input_str) {
                Ok(input) => Ok(input),
                Err(err) => {
                    eprintln!("Error deserializing input value {err}");
                    Err(err)
                }
            }
        }
    }
    fn deserialize_key_impl(
        input_str: Option<String>,
    ) -> Result<Option<Vec<u8>>, String> {
        let Some(input_str) = input_str else {
            return Ok(None);
        };
        let input = input_str.as_bytes();
        Ok(Some(input.to_vec()))
    }
    fn deserialize_input_impl(input_str: String) -> Result<String, String> {
        let mut input_str = input_str;
        let bytes = unsafe { input_str.as_mut_vec() };
        simd_json::serde::from_slice(bytes).map_err(|err| err.to_string())
    }
}
