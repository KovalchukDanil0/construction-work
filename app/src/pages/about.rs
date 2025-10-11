use leptos::prelude::*;
use leptos_router::{LazyRoute, lazy_route};

pub struct AboutPage;

#[lazy_route]
impl LazyRoute for AboutPage {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! {
            <p>"About"</p>
        }
        .into_any()
    }
}
