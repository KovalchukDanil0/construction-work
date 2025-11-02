use crate::components::Link;
use leptos::prelude::*;

struct NavEntry {
    href: &'static str,
    text: &'static str,
}

const NAV_ENTRIES: [NavEntry; 3] = [
    NavEntry {
        href: "/",
        text: "Home",
    },
    NavEntry {
        href: "about",
        text: "About",
    },
    NavEntry {
        href: "contact-us",
        text: "Contact Us",
    },
];

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="flex flex-row gap-3 justify-center items-center">
            <For
                each={|| NAV_ENTRIES.into_iter()}
                key={|state| state.href}
                let(NavEntry { href, text })
            >
                <Link class="aria-[current=page]:hidden" href>
                    {text}
                </Link>
            </For>
        </nav>
    }
}
