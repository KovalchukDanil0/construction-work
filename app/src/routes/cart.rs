use leptos::prelude::*;
use leptos_router::{LazyRoute, lazy_route};

pub struct CartPage;

#[lazy_route]
impl LazyRoute for CartPage {
    fn data() -> Self {
        Self
    }

    fn view(this: Self) -> AnyView {
        view! { <h1>"Cart"</h1> }.into_any()
    }
}
