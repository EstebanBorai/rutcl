use leptos::{component, view, IntoView};

use crate::components::section::Section;

#[component]
pub fn Motivation() -> impl IntoView {
    view! {
        <Section title="Motivation">
            <p>
                Provide a speed, robust and easy to use parser for the Chilean National ID (RUT)
                for Rust applications.{" "}Which supports multiple formatting and validation options.
            </p>
            <p>
                Given that the RUT is a common identifier in Chile, {" it's "} important to have a
                reliable and easy to use parser for it.
            </p>
            <p>
                An advantage of using a Rust parser is that it can be used in any Rust application
                including server and client, this site is using the parser to validate the RUTs
                as part of this website demos.{" "}This means your applications will use the same logic
                to parse RUTs in both codebases, client and server!
            </p>
        </Section>
    }
}
