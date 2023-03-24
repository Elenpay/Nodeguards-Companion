use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html};
use yew_router::prelude::use_navigator;

use crate::{switch::Mnemonic, Route};

#[function_component(ImportWallet)]
pub fn import_wallet() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick_import = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Mnemonic {
                action: Mnemonic::Import,
            });
        })
    };

    let onclick_generate = Callback::from(move |_: MouseEvent| {
        navigator.push(&Route::Mnemonic {
            action: Mnemonic::Generate,
        });
    });

    html! {
        <>
            <h class="title">{"Import your wallet"}</h>
            <div class="import-buttons">
                <button onclick={onclick_generate}>{"Generate Mnemonic"}</button>
                <button onclick={onclick_import}>{"Import from Mnemonic"}</button>
                <button>{"Import from Master Private Key (TODO)"}</button>
            </div>
        </>
    }
}
