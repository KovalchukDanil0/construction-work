use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Link(
    #[prop(into)] href: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <A href={href} attr:class={class}>{children()}</A>
    }
}
