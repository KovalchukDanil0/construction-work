use app::App;
use leptos::mount::hydrate_lazy;
use log::Level;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hydrate() -> Result<(), JsError> {
    // initializes logging using the `log` crate
    console_log::init_with_level(Level::Debug)
        .map_err(|_| JsError::new("Console Log Init Error"))?;
    console_error_panic_hook::set_once();

    hydrate_lazy(App);
    Ok(())
}
