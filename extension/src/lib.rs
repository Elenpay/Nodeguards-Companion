pub mod context;
pub mod utils;
pub mod components;
pub mod features;
pub mod switch;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use switch::{switch, Route};
use context::UserContextProvider;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <Switch<Route> render={switch} />
            </UserContextProvider>
        </BrowserRouter> 
    }
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
fn main() {
    yew::Renderer::<App>::new().render();
}