use leptos::prelude::*;

#[component]
pub fn Billboard(
    #[prop(into)] alt: String,
    #[prop(into)] src: String,
    #[prop(into)] title: String,
    #[prop(optional, into)] description: Option<String>,
) -> impl IntoView {
    view! {
        <div class="relative text-center w-full">
            <img src={src} alt={alt} />
            <div class="absolute bottom-1/4 left-1/2 -translate-1/2 text-black text-shadow-lg">
                <h2>{title}</h2>
                <ShowLet
                    some={move || description.clone()}
                    let:value
                >
                    <p>{value}</p>
                </ShowLet>
            </div>
       </div>
    }
}
