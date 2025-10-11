use crate::components::{Button, Link};
use icondata::HiMagnifyingGlassMinusOutlineLg;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;
use urlencoding::encode;

#[component]
pub fn Header() -> impl IntoView {
    let navigate = use_navigate();

    let search_signal = RwSignal::new(String::new());

    let search_click = move |_| {
        let mut search_query = search_signal.get();
        if search_query.is_empty() {
            return;
        }

        search_query = format!("/search?q={}", encode(&search_query));
        navigate(&search_query, Default::default())
    };

    view! {
        <div class="w-full h-16 flex flex-row gap-3 justify-around items-center">
            <Link href="/" class="contents">
                <img src="/steel-building.jpg" class="object-contain h-full" />
            </Link>

            <div class="relative">
                <input bind:value={search_signal} type="search" class="w-full" name="search" placeholder="Search here"/>
                <Button class="absolute right-0 top-0" {..} on:click={search_click} type="submit">"Search"</Button>
            </div>

            <Link href="/auth">
                <Icon icon=HiMagnifyingGlassMinusOutlineLg />
            </Link>

            <Link href="/wishlist">"Wishlist"</Link>
            <Link href="/cart">"Cart"</Link>
        </div>
    }
}
