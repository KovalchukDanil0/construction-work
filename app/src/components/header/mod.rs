mod navigation;

use crate::components::{Button, Link, Input, Logo};
use icondata::{FaCartShoppingSolid, FaMagnifyingGlassSolid, FaUserSolid};
use leptos::{ev, html, prelude::*};
use leptos_icons::Icon;
use leptos_router::hooks::use_navigate;
use navigation::Navigation;
use urlencoding::encode;
use tw_merge::tw_merge;

#[component]
fn Search() -> impl IntoView {
    let navigate = use_navigate();
    let search_signal = RwSignal::new(String::new());

    let input_element = NodeRef::<html::Div>::new();

    let search_click = move |_| {
        let mut search_query = search_signal.get();
        if search_query.is_empty() {
            return;
        }

        if let Some(input_element) = input_element.get() {
            let _ = input_element.hide_popover();
        }

        search_query = format!("/search?q={}", encode(&search_query));
        navigate(&search_query, Default::default())
    };

    view! {
        <Button {..} popovertarget="search-popover">
            <Icon icon=FaMagnifyingGlassSolid />
        </Button>

        <div class="top-1/4 left-1/2 -translate-1/2 w-1/2 px-8 py-4 bg-gray-700" id="search-popover" node_ref={input_element} popover>
            <div class="flex flex-row gap-3 items-center justify-around">
                <Input
                    bind_value={search_signal}
                    class="w-full"
                    placeholder="Search here"
                />
                <Button {..} on:click={search_click}>"Search"</Button>
            </div>
        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let (show_header, set_show_header) = signal(true);
    let (last_scroll_top, set_last_scroll_top) = signal(f64::default());

    let handle = window_event_listener(ev::scroll, move |_| {
        let last_scroll_top = last_scroll_top.get();

        let scroll_y = window().scroll_y().unwrap_or_default();
        let is_scroll_top = scroll_y < last_scroll_top;
        set_show_header.set(is_scroll_top);

        let scroll_top = if scroll_y > 0. { scroll_y } else { 0. };
        set_last_scroll_top.set(scroll_top);
    });

    on_cleanup(move || handle.remove());

    view! {
        <header
            class={move || tw_merge!(
                "sticky top-0 left-0 z-50 flex flex-row gap-3 justify-around items-center py-4 md:py-8 bg-black",
                if show_header.get() {"visible"} else {"invisible"}
            )}
        >
            <Logo />
            <Navigation />

            <div class="flex flex-row gap-6 items-center justify-center">
                <Search />
                <Link href="/auth">
                    <Icon icon=FaUserSolid />
                </Link>
                <Link href="/cart">
                    <Icon icon=FaCartShoppingSolid />
                </Link>
            </div>
        </header>
    }
}
