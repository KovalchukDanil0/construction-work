mod navigation;

use crate::components::{Button, Link};
use icondata::{FaCartShoppingSolid, FaMagnifyingGlassSolid, FaUserSolid};
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;
use navigation::Navigation;
use urlencoding::encode;

#[component]
fn Logo() -> impl IntoView {
    view! {
        <Link href="/" class="contents">
            <img src="/steel-building.jpg" class="object-contain h-full" />
        </Link>
    }
}

#[component]
fn Search() -> impl IntoView {
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
        <Button {..} popovertarget="search-popover">
            <Icon icon=FaMagnifyingGlassSolid />
        </Button>

        <div class="top-1/4 left-1/2 -translate-1/2" id="search-popover" popover>
            <input
                bind:value={search_signal}
                class="w-full"
                placeholder="Search here"
            />
            <Button {..} on:click={search_click}>"Search"</Button>
        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="sticky top-0 left-0 h-36 z-50 flex flex-row gap-3 justify-around items-center py-8">
            <Logo />
            <Navigation />
            <Search />

            <Link href="/auth">
                <Icon icon=FaUserSolid />
            </Link>

            <Link href="/cart">
                <Icon icon=FaCartShoppingSolid />
            </Link>
        </header>
    }
}
