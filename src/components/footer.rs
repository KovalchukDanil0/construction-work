use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="flex flex-row gap-3">
            <div class="flex flex-col gap-3">
                <p>"Title"</p>
                <p>"Text"</p>
                <p>"Text"</p>
            </div>

            <div class="flex flex-col gap-3">
                <p>"Title"</p>
                <p>"Text"</p>
                <p>"Text"</p>
            </div>

            <div class="flex flex-col gap-3">
                <p>"Title"</p>
                <p>"Text"</p>
                <p>"Text"</p>
            </div>
        </div>
    }
}
