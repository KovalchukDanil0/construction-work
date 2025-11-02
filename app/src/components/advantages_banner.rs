use icondata::{FaCheckSolid, FaClockSolid, FaTruckSolid, Icon};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
fn IconContainer(#[prop(into)] text: String, #[prop(into)] icon: Icon) -> impl IntoView {
    view! {
        <div class="flex flex-row gap-3 justify-center items-center">
            <Icon icon />
            <p class="text-sm">{text}</p>
        </div>
    }
}

#[component]
pub fn AdvantagesBanner() -> impl IntoView {
    view! {
        <div class="hidden flex-row gap-3 justify-around items-center md:flex">
            <IconContainer text="Free shipping on orders over $350 excl. VAT" icon={FaTruckSolid} />
            <IconContainer
                text="Ordered before 1 PM (Mon-Fri), dispatched the same day if in stock"
                icon={FaClockSolid}
            />
            <IconContainer
                text="For more than 25 years the official importer of Construction Work"
                icon={FaCheckSolid}
            />
        </div>
    }
}
