use crate::components::{Image, Link};
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Logo(#[prop(optional, into)] class: Option<String>) -> impl IntoView {
    view! {
        <Link href="/" class="contents">
            <Image
                src="/steel-building.jpg"
                alt="Steel Building"
                class={tw_merge!("object-contain h-15 md:h-20", class)}
            />
        </Link>
    }
}
