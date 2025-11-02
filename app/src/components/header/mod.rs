mod navigation;
mod search;

use crate::components::{Link, Logo};
use icondata::{FaCartShoppingSolid, FaUserSolid};
use leptos::{ev, prelude::*};
use leptos_icons::Icon;
use navigation::Navigation;
use search::Search;
use tailwind_fuse::tw_merge;

#[component]
pub fn Header() -> impl IntoView {
    let (show_header, set_show_header) = signal(true);
    let (last_scroll_top, set_last_scroll_top) = signal(0.);

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
        <header class={move || {
            tw_merge!(
                "sticky top-0 left-0 z-50 flex flex-row gap-3 justify-around items-center py-4 md:py-8 bg-black",
                if show_header.get() {"visible"} else {"invisible"}
            )
        }}>
            <Logo />
            <Navigation />

            <div class="flex flex-row gap-6 justify-center items-center">
                <Search />
                <Link href="auth">
                    <Icon icon={FaUserSolid} />
                </Link>
                <Link href="cart">
                    <Icon icon={FaCartShoppingSolid} />
                </Link>
            </div>
        </header>
    }
}
