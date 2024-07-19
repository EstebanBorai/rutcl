use leptos::{component, view, IntoView};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="text-sm text-emerald-500 flex justify-end sticky p-4 border-b border-zinc-800">
            <div>
                <a href="https://github.com/EstebanBorai/rutcl" target="_blank">GitHub</a>
            </div>
        </header>
    }
}
