use app::App;
use wasm_bindgen::prelude::*;
use leptos::mount::hydrate_lazy;

#[wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    hydrate_lazy(App);
}
