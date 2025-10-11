use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Button<'a>(children: Children, #[prop(optional)] class: &'a str) -> impl IntoView {
    view! {
        <button class={tw_merge!("hover:cursor-pointer", class)}>{children()}</button>
    }
}
