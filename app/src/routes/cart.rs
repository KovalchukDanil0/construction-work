use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

pub struct CartPage;

#[lazy_route]
impl LazyRoute for CartPage {
    fn data() -> Self {
        Self
    }

    fn view(this: Self) -> AnyView {
        view! {
            <h1>"Cart"</h1>
        }.into_any()
    }
}
