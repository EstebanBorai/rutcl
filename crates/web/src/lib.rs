mod components;
mod sections;

use components::header::Header;
use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Title};

use self::components::navbar::NavBar;
use self::sections::create_rut::CreateRut;
use self::sections::hero::Hero;
use self::sections::installation::Installation;
use self::sections::motivation::Motivation;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="RUT Chile | Chilean National ID (RUT) Parser for Rust"/>
        <div class="grid md:grid-cols-[250px,auto] bg-zinc-950 text-gray-50 min-h-screen">
            <NavBar/>
            <div class="h-screen oveflow-hidden">
                <Header/>
                <main class="h-[calc(100vh-60px)] overflow-y-scroll">
                    <Hero/>
                    <Motivation/>
                    <Installation/>
                    <CreateRut/>
                </main>
            </div>
        </div>
    }
}
