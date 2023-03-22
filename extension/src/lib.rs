pub mod context;
pub mod utils;
pub mod components;
pub mod features;
pub mod switch;

use serde::Deserialize;
use utils::events::{EventManager, State};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*};
use switch::{switch, Route};

#[function_component]
fn App() -> Html {
    html! {
        <div class="app">
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

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

#[wasm_bindgen]
extern { fn pastePSBT(value: JsValue); }