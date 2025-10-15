use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Button(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <button class={tw_merge!("hover:cursor-pointer", class)}>{children()}</button>
    }
}
