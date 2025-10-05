use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Button(
    children: Children,
    #[prop(optional)]
    class: &'static str
) -> impl IntoView {
    view! {
        <button class={tw_merge!("hover:cursor-pointer", class)}>{children()}</button>
    }
}
