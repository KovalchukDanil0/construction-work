use leptos::prelude::*;
use crate::components::{Image, Link};

#[component]
pub fn Billboard(
    #[prop(into)] alt: String,
    #[prop(into)] src: String,
    #[prop(into)] title: String,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] button_text: Option<String>,
) -> impl IntoView {
    view! {
        <div class="relative text-center w-full h-150 md:h-120">
            <Image class="object-cover size-full" src={src} alt={alt} />
            <div class="absolute md:bottom-1/4 bottom-1/6 left-1/2 -translate-1/2 text-black text-shadow-lg">
                <h2>{title}</h2>

                <ShowLet
                    some={move || description.clone()}
                    let:description
                >
                    <p>{description}</p>
                </ShowLet>

                <ShowLet
                    some={move || button_text.clone().zip(href.clone())}
                    let((href, button_text))
                >
                    <Link href>{button_text}</Link>
                </ShowLet>
            </div>
       </div>
    }
}
