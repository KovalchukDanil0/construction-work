use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

pub struct AboutPage;

#[lazy_route]
impl LazyRoute for AboutPage {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! {
            <p>"About"</p>
        }.into_any()
    }
}
