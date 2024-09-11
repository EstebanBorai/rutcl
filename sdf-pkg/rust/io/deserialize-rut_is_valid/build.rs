fn main() {
    let bindings = r#"use wit_bindgen::generate;
    generate!({
        world: "deserialize-rut-is-valid-world",
        path: ".wit",
        additional_derives: [serde::Serialize, serde::Deserialize],
        generate_all,
        generate_unused_types: true
    });"#;
    std::fs::write("src/bindings.rs", bindings).expect("Unable to write bindings.rs");
}