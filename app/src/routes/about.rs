use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

pub struct AboutPage;

#[lazy_route]
impl LazyRoute for AboutPage {
    fn data() -> Self {
        Self
    }

    fn view(this: Self) -> AnyView {
        view! {
            <h1>"About"</h1>
        }.into_any()
    }
}
