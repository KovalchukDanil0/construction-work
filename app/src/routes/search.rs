use leptos::prelude::*;
use leptos_router::{LazyRoute, hooks::query_signal, lazy_route};

pub struct SearchPage;

#[lazy_route]
impl LazyRoute for SearchPage {
    fn data() -> Self {
        Self
    }

    fn view(this: Self) -> AnyView {
        let (query, _set_query) = query_signal::<String>("q");

        view! {
            <h1>"Search"</h1>

            <p>{query.get()}</p>
        }
        .into_any()
    }
}
