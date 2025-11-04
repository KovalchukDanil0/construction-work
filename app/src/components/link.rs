use leptos::prelude::*;
use leptos_router::components::A;
use tailwind_fuse::{AsTailwindClass, TwVariant, tw_merge};

#[derive(TwVariant)]
pub enum Variant {
    #[tw(default, class = "")]
    Default,
    #[tw(class = "font-medium text-blue-600 dark:text-blue-500")]
    Blue,
}

#[component]
pub fn Link(
    children: Children,
    #[prop(default = Variant::Default)] variant: Variant,
    #[prop(into)] href: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <A href {..} class={tw_merge!("hover:underline", variant.as_class(), class)}>
            {children()}
        </A>
    }
}
