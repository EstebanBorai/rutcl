mod components;

use leptos::{component, create_signal, view, IntoView, SignalGet, SignalSet};
use leptos_meta::{provide_meta_context, Title};

use rutcl::Rut;

use self::components::header::Header;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (random_rut_getter, random_rut_setter) = create_signal(Rut::random());

    view! {
        <Title text="RUT Chile | Chilean National ID (RUT) Parser for Rust"/>
        <Header/>
        <div class="relative flex justify-center mx-auto max-w-8xl sm:px-2 lg:px-8 xl:px-12">
            // <NavBar />
            <main class="flex-auto max-w-2xl min-w-0 px-4 py-10 lg:max-w-none lg:pr-0 lg:pl-8 xl:px-16">
                <section class="py-10 flex flex-col items-center justify-center space-y-6">
                    <h2 class="text-2xl text-center font-bold tracking-tight text-slate-900">
                        Generates random RUTs and formats them
                    </h2>
                    <article class="flex flex-col justify-center items-center space-y-2">
                        <span class="py-2 border border-gray-50 rounded-md shadow-md">
                            <strong class="px-2">Dots Formatting</strong>
                            <code class="px-2">
                                {move || random_rut_getter.get().format(rutcl::Format::Dots)}
                            </code>
                        </span>
                        <span class="py-2 border border-gray-50 rounded-md shadow-md">
                            <strong class="px-2">Dash Formatting</strong>
                            <code class="px-2">
                                {move || random_rut_getter.get().format(rutcl::Format::Dash)}
                            </code>
                        </span>
                        <span class="py-2 border border-gray-50 rounded-md shadow-md">
                            <strong class="px-2">Sans Formatting</strong>
                            <code class="px-2">
                                {move || random_rut_getter.get().format(rutcl::Format::Sans)}
                            </code>
                        </span>
                    </article>
                    <button
                        class="px-4 py-2 text-sm font-semibold rounded bg-slate-900 text-slate-50 transition ease-in-out delay-75 hover:scale-105 duration-200"
                        type="button"
                        on:click=move |_| {
                            random_rut_setter.set(Rut::random());
                        }
                    >
                        Generate random RUT
                    </button>
                </section>
            </main>
        </div>
    }
}
