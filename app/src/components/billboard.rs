use crate::components::{Image, Link};
use leptos::prelude::*;

#[component]
pub fn Billboard(
    #[prop(into)] alt: String,
    #[prop(into)] src: String,
    #[prop(into)] title: String,
    /// (href, text)
    #[prop(optional)]
    button: Option<(&'static str, &'static str)>,
    #[prop(optional, into)] description: Option<String>,
) -> impl IntoView {
    view! {
        <div class="relative w-full text-center h-150 md:h-120">
            <Image class="object-cover size-full" src alt />
            <div class="absolute left-1/2 text-black md:bottom-1/4 bottom-1/6 -translate-1/2 text-shadow-lg">
                <h2>{title}</h2>

                <ShowLet some={move || description.clone()} let:description>
                    <p>{description}</p>
                </ShowLet>

                <ShowLet some={move || button.clone()} let((href, text))>
                    <Link href>{text}</Link>
                </ShowLet>
            </div>
        </div>
    }
}
