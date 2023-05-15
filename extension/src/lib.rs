#![warn(
     clippy::all,
     //clippy::pedantic,
     clippy::nursery,
)]

pub mod app;
pub mod components;
pub mod context;
pub mod features;
pub mod switch;
pub mod utils;

use anyhow::Result;
use app::App;
use serde::Deserialize;
use utils::{
    casts::{call_fn_str, call_fn_str_async, call_fn_to_bool, call_fn_to_str_async},
    events::{EventManager, State},
};
use wasm_bindgen::prelude::*;

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
    call_fn_str("pastePSBT", psbt)
}

#[allow(clippy::future_not_send)]
pub async fn save_password(password: &str) -> Result<()> {
    call_fn_str_async("savePassword", password).await
}

#[allow(clippy::future_not_send)]
pub async fn get_password() -> Result<String> {
    call_fn_to_str_async("getPassword").await
}

pub fn session_exists() -> Result<bool> {
    call_fn_to_bool("sessionExists")
}
