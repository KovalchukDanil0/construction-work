use leptos::prelude::*;
use tw_merge::tw_merge;

#[derive(Default)]
pub enum Directions {
    #[default]
    Col,
    ColRev,
    Row,
    RowRev,
}

#[component]
pub fn Input(
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] direction: Directions,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] bind_value: RwSignal<String>,
) -> AnyView {
    let direction = match direction {
        Directions::Col => "flex-col",
        Directions::ColRev => "flex-col-reverse",
        Directions::Row => "flex-row",
        Directions::RowRev => "flex-row-reverse",
    };

    let input = view! {
        <input name={name} placeholder={placeholder} bind:value={bind_value} class="border border-white" size="30" />
    };

    if let Some(label) = label {
        view! {
            <label class={tw_merge!("flex", direction, class)}>
                {label}
                {input}
            </label>
        }
        .into_any()
    } else {
        input.into_any()
    }
}
