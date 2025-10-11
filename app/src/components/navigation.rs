use crate::components::Link;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

const NAV_ENTRIES: [(&str, &str); 3] = [("/", "Home"), ("/about", "About"), ("/test", "Test")];

#[component]
pub fn Navigation() -> impl IntoView {
    let location = use_location();

    view! {
        <nav class="w-full flex flex-row gap-3 justify-center items-center">
            <For each={move || NAV_ENTRIES.iter().filter(move |(href, _)| {
                *href != location.pathname.get()
            })} key={|state| state.0} let((href, text))>
                <Link href={href}>{*text}</Link>
            </For>
        </nav>
    }
}
