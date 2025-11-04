use icondata::Icon;
use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::tw_merge;

#[component]
pub fn CardFeature(
    children: Children,
    icon: Icon,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <div class={tw_merge!(
            "flex flex-col gap-3 items-center justify-center text-center", class
        )}>
            <Icon icon {..} class="size-10" />
            {children()}
        </div>
    }
}
