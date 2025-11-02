use crate::components::{Button, Input};
use icondata::FaMagnifyingGlassSolid;
use leptos::{html, prelude::*};
use leptos_icons::Icon;

#[server]
pub async fn search_action(query: String) -> Result<(), ServerFnError> {
    use leptos_axum::redirect;
    use urlencoding::encode;

    let encoded_query = encode(query.as_str());

    let page = format!("/search?q={}", encoded_query);
    redirect(page.as_str());
    Ok(())
}

#[component]
pub fn Search() -> impl IntoView {
    let search_action = ServerAction::<SearchAction>::new();
    let popover_ref = NodeRef::<html::Div>::new();

    let hide_popover = move |_| {
        if let Some(popover_ref) = popover_ref.get() {
            popover_ref.hide_popover().ok();
        }
    };

    view! {
        <Button {..} popovertarget="search-popover">
            <Icon icon={FaMagnifyingGlassSolid} />
        </Button>

        <div
            class="left-1/2 top-1/4 py-4 px-8 w-1/2 bg-gray-700 -translate-1/2"
            id="search-popover"
            node_ref={popover_ref}
            popover
        >
            <ActionForm
                action={search_action}
                {..}
                class="flex flex-row gap-3 justify-around items-center"
                on:submit={hide_popover}
            >
                <Input name="query" class="w-full" placeholder="Search here" />
                <Button {..} type="submit">
                    "Search"
                </Button>
            </ActionForm>

        </div>
    }
}
