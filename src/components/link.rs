use leptos::prelude::*;

#[component]
pub fn Link(
    href: &'static str,
    children: Children
) -> impl IntoView {
    view! {
        <a href={href}>{children()}</a>
    }
}
