use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="Not Found" />

        <h1>"Page not found."</h1>
    }
}
