use leptos::prelude::*;

#[component]
pub fn Image(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <img src={src} alt={alt} class={class} /> }
}
