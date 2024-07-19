use leptos::{component, view, Children, IntoView};

#[component]
pub fn Section(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <section class="p-4">
            <h2 id={title.to_ascii_lowercase().replace(' ', "-")}>{title}</h2>
            <div>
                {children()}
            </div>
        </section>
    }
}
