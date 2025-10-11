use leptos::prelude::*;

#[component]
pub fn Billboard(
    #[prop(into)] alt: String,
    #[prop(into)] src: String,
    #[prop(into)] title: String,
    #[prop(optional, into)] description: String,
) -> impl IntoView {
    view! {
        <div class="relative text-center">
            <img src={src} alt={alt} class="w-full" />
            <div class="absolute bottom-1/4 left-1/2 -translate-1/2 text-black text-shadow-lg">
                <h2>{title}</h2>
                {if description.is_empty() {
                    Some(view! {
                        <p>
                            {description}
                        </p>
                    })
                } else {
                    None
                }}
            </div>
       </div>
    }
}
