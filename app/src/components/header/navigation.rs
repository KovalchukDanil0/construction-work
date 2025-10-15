use crate::components::Link;
use leptos::prelude::*;

const NAV_ENTRIES: [(&str, &str); 3] = [("/", "Home"), ("/about", "About"), ("/test", "Test")];

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="flex flex-row gap-3 justify-center items-center">
            <For
                each={|| NAV_ENTRIES.into_iter()}
                key={|state| state.0}
                let((href, text))
            >
                <Link class="aria-[current=page]:hidden" href={href}>{text}</Link>
            </For>
        </nav>
    }
}
