use crate::components::icons::github::GitHub;
use leptos::{component, view, IntoView};

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <aside class="relative hidden md:block col-start-1 col-end-1 h-screen border-r border-zinc-800">
            <div class="h-[60px] p-4 border-b border-transparent flex justify-between items-center">
            <a class="flex justify-center items-center h-8 w-8" href="https://github.com/EstebanBorai/rutcl" target="_blank">
                <h1 class="hover:text-emerald-600">rutcl</h1>
                </a>
                <a class="flex justify-center items-center h-8 w-8" href="https://github.com/EstebanBorai/rutcl" target="_blank">
                <GitHub class="size-5" />
                </a>
            </div>
            <nav class="navbar p-4 space-y-2 h-[calc(100vh-120px)] overflow-y-auto">
                <a class="link" href="/#motivation">Motivation</a>
                <a class="link" href="/#installation">Installation</a>
                <span class="section-divider">Usage</span>
                <a class="link" href="/#create-rut">Create RUT</a>
            </nav>
            <footer class="h-[60px] bg-zinc-950 absolute p-4 bottom-0 w-full">
                <small class="text-xs text-center block text-gray-400">
                    Developed with Rust and Coffee by{" "}
                    <a href="https://github.com/EstebanBorai" target="_blank" class="text-emerald-600 underline">
                        Esteban Borai
                    </a>
                </small>
            </footer>
        </aside>
    }
}
