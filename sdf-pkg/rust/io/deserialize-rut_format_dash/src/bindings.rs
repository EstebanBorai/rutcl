use wit_bindgen::generate;
    generate!({
        world: "deserialize-rut-format-dash-world",
        path: ".wit",
        additional_derives: [serde::Serialize, serde::Deserialize],
        generate_all,
        generate_unused_types: true
    });