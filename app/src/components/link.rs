use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Link<'a>(
    children: Children,
    href: &'static str,
    #[prop(optional)] class: &'a str,
) -> impl IntoView {
    view! {
        <A href={href} attr:class={class}>{children()}</A>
    }
}
