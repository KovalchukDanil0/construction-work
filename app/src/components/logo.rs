use leptos::prelude::*;
use crate::components::{Image, Link};

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <Link href="/" class="contents">
            <Image src="/steel-building.jpg" alt="Steel Building" class="object-contain h-15 md:h-20" />
        </Link>
    }
}
