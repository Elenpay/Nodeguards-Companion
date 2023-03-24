pub mod app;
pub mod components;
pub mod context;
pub mod features;
pub mod switch;
pub mod utils;

use anyhow::{anyhow, Result};
use app::App;
use js_sys::{Function, Reflect};
use serde::Deserialize;
use switch::Route;
use utils::events::{EventManager, State};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(start)]
fn main() {
    yew::Renderer::<App>::new().render();
}

#[wasm_bindgen]
#[derive(Deserialize, Debug, Default, Clone)]
pub struct OperationRequestData {
    psbt: Option<String>,
    request_type: Option<String>,
    amount: Option<String>,
}

#[wasm_bindgen]
pub fn approve_psbt(value: JsValue) {
    if let Ok(psbt) = serde_wasm_bindgen::from_value::<OperationRequestData>(value) {
        EventManager::call("approve_psbt", State::new(psbt));
    }
}

pub fn paste_psbt(psbt: &str) -> Result<()> {
    let window = window().ok_or(anyhow!("Window not found"))?;
    let paste_value = Reflect::get(&window, &JsValue::from_str("pastePSBT"))
        .map_err(|_| anyhow!("Error while getting JS function"))?;

    let paste_function = paste_value
        .dyn_ref::<Function>()
        .ok_or(anyhow!("Cast from JS to Rust invalid"))?;

    paste_function
        .call1(&JsValue::undefined(), &psbt.into())
        .map(|_| ())
        .map_err(|_| anyhow!("Error while calling JS function"))
}
