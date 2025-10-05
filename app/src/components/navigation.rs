use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::components::Link;

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
            {move || {
                let pathname = location.pathname.get();

                NAV_ENTRIES.into_iter().filter_map(|(href, text)| {
                    if href != pathname {
                        Some(view! {
                            <Link href={href}>{text}</Link>
                        })
                    } else {
                        None
                    }
                }).collect_view()
            }}
        </nav>
    }
}
