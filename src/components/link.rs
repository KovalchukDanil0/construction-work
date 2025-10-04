use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Link(
    href: &'static str,
    children: Children
) -> impl IntoView {
    view! {
        <A href={href}>{children()}</A>
    }
}
