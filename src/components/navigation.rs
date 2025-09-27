use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

struct NavEntries<'a> {
    href: &'a str,
    text: &'a str,
}

const NAV_ENTRIES: [NavEntries; 3] = [
    NavEntries {
        href: "/",
        text: "Home",
    },
    NavEntries {
        href: "/about",
        text: "About",
    },
    NavEntries {
        href: "/test",
        text: "Test",
    },
];

#[component]
pub fn Navigation() -> impl IntoView {
    let location = use_location();

    view! {
        <div class="flex flex-row gap-3">
            {
                move || {
                    let pathname = location.pathname.get();

                    NAV_ENTRIES.into_iter().filter_map(|NavEntries { href, text }| {
                        if href != pathname {
                            Some(view! {
                                <A href={href}>{text}</A>
                            })
                        } else {
                            None
                        }
                    }).collect_view()}
            }
        </div>
    }
}
