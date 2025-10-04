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
            <img src="/steel-building.jpg" class="object-contain h-full" />

            <ActionForm action={test_action} {..} class="relative">
                <input type="search" class="w-full" name="search" placeholder="Search here"/>
                <Button {..} type="submit" class="absolute right-0 top-0">"Search"</Button>
            </ActionForm>

            <Link href="/auth">
                <Icon icon=HiMagnifyingGlassMinusOutlineLg />
            </Link>

            <Link href="/wishlist">"Wishlist"</Link>
            <Link href="/cart">"Cart"</Link>
        </div>
    }
}
