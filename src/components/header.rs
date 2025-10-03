use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::Form;
use crate::components::{Button, Link};
use icondata::BsFolder;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="w-full h-16 flex flex-row gap-3 justify-around items-center">
            <img src="/steel-building.jpg" class="object-contain h-full" />

            <Form method="GET" action="" {..} class="relative">
                <input class="w-full" type="search" name="search" placeholder="Search here"/>
                <Button {..} class="absolute right-0 top-0" type="submit">"Search"</Button>
            </Form>

            <Link href="/auth">
                <Icon icon=BsFolder />
            </Link>

            <Link href="/wishlist">"Wishlist"</Link>
            <Link href="/cart">"Cart"</Link>
        </div>
    }
}
