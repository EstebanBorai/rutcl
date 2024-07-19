use leptos::{component, view, IntoView};

use crate::components::section::Section;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <Section title="RUT Chile">
            <div>
                <p align="center">"Chilean National ID (RUT) Parser"</p>
            </div>
            <div class="md:space-y-0 flex flex-wrap justify-center">
                <img class="mx-2 my-2 md:my-0" src="https://img.shields.io/crates/v/rutcl.svg" alt="Crates.io" />
                <img class="mx-2 my-2 md:my-0" src="https://docs.rs/rutcl/badge.svg" alt="Documentation" />
                <img class="mx-2 my-2 md:my-0" src="https://github.com/EstebanBorai/rutcl/workflows/build/badge.svg" alt="Build" />
                <img class="mx-2 my-2 md:my-0" src="https://github.com/EstebanBorai/rutcl/workflows/clippy/badge.svg" alt="Clippy" />
                <img class="mx-2 my-2 md:my-0" src="https://github.com/EstebanBorai/rutcl/workflows/fmt/badge.svg" alt="Formatter" />
                <img class="mx-2 my-2 md:my-0" src="https://github.com/EstebanBorai/rutcl/workflows/test/badge.svg" alt="Tests" />
            </div>
        </Section>
    }
}
