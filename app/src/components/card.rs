use crate::components::Link;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Card(
    #[prop(into)] img: String,
    #[prop(into)] alt: String,
    #[prop(into)] title: String,
    #[prop(optional)] button: Option<(&'static str, &'static str)>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <div class={tw_merge!("relative text-left text-white h-64", class)}>
            <img class="object-cover rounded-md size-full" alt={alt} src={img} />
            <div class="absolute top-6 left-6">
                <p>{title}</p>
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
