use leptos::{component, create_signal, view, IntoView, SignalGet, SignalSet};
use rutcl::{Format, Rut};

use crate::components::section::Section;

#[component]
pub fn CreateRut() -> impl IntoView {
    let (random_rut_reader, random_rut_writer) = create_signal(Rut::random());
    let (random_in_range_reader, random_in_range_writer) = create_signal(Rut::random_in_range(10_000_000..15_000_000).unwrap());

    let randomize = move |_| {
        random_rut_writer.set(Rut::random());
        random_in_range_writer.set(Rut::random_in_range(10_000_000..15_000_000).unwrap());
    };

    view! {
        <Section title="Create RUT">
            <p>An instance of <code>Rut</code> can be created either by using <code>"std::str::FromStr"</code> trait.</p>
            <code class="my-4">
                let rut = Rut::from_str("12345678-9");
            </code>
            <p>Validations to the provided string will apply at the moment of creating the RUT instance.</p>
            <p>Random RUT values can also be created using the <code>Rut::random()</code> associated function.</p>
            <code class="my-4">
                let rut = Rut::random();
            </code>
            <h3>Example</h3>
                <p class="bg-gray-900 p-4 font-mono rounded-md shadow-md mb-4">{move || random_rut_reader.get().unwrap().format(Format::Dots)}</p>
            <button type="button" on:click={randomize}>Generate</button>
        </Section>
        <Section title="Random in Range">
            <p>Random RUT values can also be created within a range using the <code>Rut::random_in_range()</code> associated function.</p>
            <code class="my-4">
                let rut = Rut::random_in_range(10_000_000..15_000_000);
            </code>
            <h3>Example</h3>
                <p class="bg-gray-900 p-4 font-mono rounded-md shadow-md mb-4">{move || random_in_range_reader.get().format(Format::Dots)}</p>
            <button type="button" on:click={randomize}>Generate</button>
        </Section>
    }
}
