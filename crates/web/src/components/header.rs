use leptos::{component, view, IntoView};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-50 flex items-center justify-between px-3 py-2 border-b shadow-lg bg-white/90 backdrop-blur-sm border-slate-400/40">
            <div class="flex items-center flex-grow basis-0">
              <a href="" class="text-lg font-semibold tracking-tight text-slate-900">
                rutcl
              </a>
            </div>

            <div class="items-center justify-end flex-grow hidden basis-0 md:flex">
              <a href="https://github.com/EstebanBorai/rutcl" class="px-4 py-2 text-sm font-semibold rounded bg-slate-900 text-slate-50 transition ease-in-out delay-75 hover:scale-105 duration-200">
                Visit GitHub
              </a>
            </div>
          </header>
    }
}
