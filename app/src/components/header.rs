use leptos::prelude::*;
use leptos_icons::Icon;
use crate::components::{Button, Link};
use icondata::HiMagnifyingGlassMinusOutlineLg;

#[server]
async fn test_action() -> Result<(), ServerFnError> {
    Ok(())
}

#[component]
pub fn Header() -> impl IntoView {
    let test_action = ServerAction::<TestAction>::new();

    view! {
        <div class="w-full h-16 flex flex-row gap-3 justify-around items-center">
            <Link href="/" class="contents">
                <img src="/steel-building.jpg" class="object-contain h-full" />
            </Link>

            <ActionForm action={test_action} attr:class="relative">
                <input type="search" class="w-full" name="search" placeholder="Search here"/>
                <Button class="absolute right-0 top-0" {..} type="submit">"Search"</Button>
            </ActionForm>

            <Link href="/auth">
                <Icon icon=HiMagnifyingGlassMinusOutlineLg />
            </Link>

            <Link href="/wishlist">"Wishlist"</Link>
            <Link href="/cart">"Cart"</Link>
        </div>
    }
}
