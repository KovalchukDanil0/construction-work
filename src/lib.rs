pub mod app;
mod components;
mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    use console_error_panic_hook::set_once;
    use leptos::mount::hydrate_body;

    set_once();
    hydrate_body(App);
}
