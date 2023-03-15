use wasm_bindgen::prelude::*;
use yew::prelude::*;
pub mod features;

use features::password::InputPassword;
use features::import_seed::ImportSeed;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <ImportSeed />
            <InputPassword button_label="Sign"/>
        </> 
    }
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
fn main() {
    yew::Renderer::<App>::new().render();
}