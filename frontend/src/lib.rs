use app::App;
use leptos::mount::hydrate_lazy;
use log::Level;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

    hydrate_lazy(App);
}
