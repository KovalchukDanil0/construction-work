use leptos::prelude::*;
use tailwind_fuse::{AsTailwindClass, TwVariant, tw_merge};

#[derive(TwVariant)]
pub enum Variant {
    #[tw(default, class = "flex-col")]
    Col,
    #[tw(class = "flex-col-reverse")]
    ColRev,
    #[tw(class = "flex-row")]
    Row,
    #[tw(class = "flex-row-reverse")]
    RowRev,
}

#[component]
pub fn Input(
    #[prop(default = Variant::Col)] variant: Variant,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] r#type: Option<String>,
    #[prop(optional, into)] pattern: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] bind_value: RwSignal<String>,
    #[prop(optional)] required: Option<bool>,
) -> impl IntoView {
    view! {
        <label class={tw_merge!(
            "flex", variant.as_class(), class
        )}>
            {label}
            <input
                name={name}
                placeholder={placeholder}
                required={required}
                type={r#type}
                pattern={pattern}
                bind:value={bind_value}
                class="border border-white"
                size="30"
            />
        </label>
    }
}
