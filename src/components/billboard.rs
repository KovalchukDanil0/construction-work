use leptos::prelude::*;

#[component]
pub fn Billboard(
    children: Children,
    #[prop(default = "Billboard")]
    alt: &'static str,
) -> impl IntoView {
    view! {
        <div class="relative text-center">
            <img src="/metal-construction.jpeg" alt={alt} class="w-full" />
            <div class="absolute bottom-1/4 left-1/2 -translate-1/2 text-black text-shadow-lg">{children()}</div>
       </div>
    }
}
