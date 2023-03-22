use web_sys::MouseEvent;
use yew::{Html, function_component, html, Callback};
use yew_router::prelude::use_navigator;

use crate::Route;

#[function_component(ImportWallet)]
pub fn import_wallet() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_: MouseEvent| {
        navigator.push(&Route::Mnemonic);
    });

    html! {
        <>
            <h class="title">{"Import your wallet"}</h>
            <div class="import-buttons">
                <button {onclick}>{"Import from Mnemonic"}</button>
                <button>{"Import from Master Private Key (TODO)"}</button>
            </div>
        </>
    }
}