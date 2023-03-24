use crate::{
    switch::Route,
    utils::{events::EventManager, storage::LocalStorage},
};
use signer::storage::UserStorage;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html};
use yew_router::prelude::use_navigator;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let storage = UserStorage::read(LocalStorage::default());
    if !storage.has_password() || !storage.name.is_some() {
        navigator.push(&Route::CreateAccount);
    } else if storage.wallets.len() == 0 {
        navigator.push(&Route::ImportWallet);
    }

    let nav = navigator.clone();
    EventManager::register_callback("approve_psbt", move |data| {
        nav.push_with_state(&Route::ApprovePSBT, data);
    });

    let onclick = Callback::from(move |_: MouseEvent| {
        navigator.push(&Route::ImportWallet);
    });

    html! {
        <>
            <h class="title">{"Your Wallets"}</h>
            <select name="wallets">
                {
                    storage.wallets.iter().map(|w| {
                        let name = w.name.to_string();
                        let value = w.name.to_string();
                        html! {
                            <option value={value}>{name}</option>
                        }
                }).collect::<Html>()
                }
            </select>
            <button>{"Reveal XPRV (TODO)"}</button>
            <button {onclick}>{"Import another wallet"}</button>
        </>
    }
}
