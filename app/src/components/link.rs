use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Link(
    children: Children,
    href: &'static str,
    #[prop(optional)]
    class: &'static str
) -> impl IntoView {
    view! {
        <A href={href} attr:class={class}>{children()}</A>
    }
}
