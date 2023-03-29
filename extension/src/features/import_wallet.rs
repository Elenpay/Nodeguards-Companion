use crate::switch::ImportWalletRoute;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html};
use yew_router::prelude::use_navigator;

#[function_component(ImportWallet)]
pub fn import_wallet() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick_importseed = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&ImportWalletRoute::ImportSeed);
        })
    };

    let onclick_importxprv = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&ImportWalletRoute::ImportXPRV);
        })
    };

    let onclick_generate = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&ImportWalletRoute::GenerateSeed);
        })
    };

    let onclick_goback = Callback::from(move |_: MouseEvent| navigator.back());

    html! {
        <>
            <h class="title">{"Import your wallet"}</h>
            <div class="import-buttons">
                <button onclick={onclick_generate}>{"Generate Seed"}</button>
                <button onclick={onclick_importseed}>{"Import from Seed"}</button>
                <button onclick={onclick_importxprv}>{"Import from Private Key"}</button>
                <button class="cancel" onclick={onclick_goback}>{"Go back"}</button>
            </div>
        </>
    }
}
