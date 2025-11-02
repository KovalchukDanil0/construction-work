use leptos::prelude::*;
use tailwind_fuse::{AsTailwindClass, TwVariant, tw_merge};

#[derive(TwVariant)]
pub enum Variant {
    #[tw(default, class = "")]
    Default,
    #[tw(
        class = "py-1 px-2 bg-gray-900 rounded-sm border-solid border-2 border-gray-700 text-white hover:border-gray-500"
    )]
    Black,
}

#[component]
pub fn Button(
    #[prop(optional, into)] class: Option<String>,
    #[prop(default = Variant::Default)] variant: Variant,
    children: Children,
) -> impl IntoView {
    view! {
        <button class={tw_merge!(
            "hover:cursor-pointer", variant.as_class(), class
        )}>{children()}</button>
    }
}
