use leptos::prelude::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button class="hover:cursor-pointer">{children()}</button>
    }
}
