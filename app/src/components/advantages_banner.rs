use icondata::{FaCheckSolid, FaClockSolid, FaTruckSolid, Icon};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
fn IconContainer(#[prop(into)] text: String, #[prop(into)] icon: Icon) -> impl IntoView {
    view! {
        <div class="flex flex-row gap-3 items-center justify-center">
            <Icon icon=icon />
            <p class="text-sm">{text}</p>
        </div>
    }
}

#[component]
pub fn AdvantagesBanner() -> impl IntoView {
    view! {
        <div class="flex flex-row gap-3 items-center justify-around">
            <IconContainer text="Free shipping on orders over $350 excl. VAT" icon=FaTruckSolid />
            <IconContainer text="Ordered before 1 PM (Mon-Fri), dispatched the same day if in stock" icon=FaClockSolid />
            <IconContainer text="For more than 25 years the official importer of Evolution Power Tools" icon=FaCheckSolid />
       </div>
    }
}
