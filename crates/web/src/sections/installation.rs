use leptos::{component, view, IntoView};

use crate::components::section::Section;

#[component]
pub fn Installation() -> impl IntoView {
    view! {
        <Section title="Installation">
            <h3>Crates.io</h3>
            <p>
                <code>rutcl</code>{" it's "}available as a crate in{" "}<a href="https://crates.io/crates/rutcl" target="_blank">crates.io</a>.
            </p>
            <p>
                You can install it using:
            </p>
            <p>
                <code>cargo add rutcl</code>
            </p>
            <h3>Git</h3>
            <p>
                <code>rutcl</code>{" it's "}available as a crate in{" "}<a href="https://crates.io/crates/rutcl" target="_blank">crates.io</a>.
            </p>
            <p>
                You can install it using:
            </p>
            <p>
                <code>cargo add rutcl</code>
            </p>
        </Section>
    }
}
