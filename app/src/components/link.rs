use leptos::prelude::*;
use leptos_router::components::A;
use tailwind_fuse::{AsTailwindClass, TwVariant, tw_merge};

#[derive(TwVariant)]
pub enum Variant {
    #[tw(default, class = "font-medium text-blue-600 dark:text-blue-500")]
    Default,
    #[tw(class = "")]
    Black,
}

#[component]
pub fn Link(
    #[prop(default = Variant::Default)] variant: Variant,
    #[prop(into)] href: String,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <A href {..} class={tw_merge!("hover:underline", variant.as_class(), class)}>
            {children()}
        </A>
    }
}
