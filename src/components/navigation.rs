use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

const NAV_ENTRIES: [(&str, &str); 3] = [
    ("/", "Home"),
    ("/about", "About"),
    ("/test", "Test"),
];

#[component]
pub fn Navigation() -> impl IntoView {
    let location = use_location();

    view! {
        <nav class="w-full flex flex-row gap-3 justify-center items-center">
            {
                move || {
                    let pathname = location.pathname.get();

                    NAV_ENTRIES.into_iter().filter_map(|(href, text)| {
                        if href != pathname {
                            Some(view! {
                                <A href={href}>{text}</A>
                            })
                        } else {
                            None
                        }
                    }).collect_view()}
            }
        </nav>
    }
}
