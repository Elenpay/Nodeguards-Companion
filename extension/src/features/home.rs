use crate::{
    features::input_password_modal::InputPasswordModal,
    switch::{ImportWalletRoute, Route},
    utils::{events::EventManager, state::PasswordFor, storage::LocalStorage},
};
use signer::storage::UserStorage;
use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback, Html};
use yew_router::prelude::use_navigator;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let storage = UserStorage::read(LocalStorage::default());
    let selected_wallet = use_state(|| storage.get_default_wallet());
    let revealed_secret = use_state(String::default);
    let popup_visible = use_state(|| false);
    let derivation = use_state(String::default);

    if !storage.has_password() || storage.name.is_none() {
        navigator.push(&Route::CreateAccount);
    } else if storage.wallets.is_empty() {
        navigator.push(&ImportWalletRoute::ImportWalletHome);
    }

    let nav = navigator.clone();
    EventManager::register_callback("approve_psbt", move |data| {
        nav.push_with_state(&Route::ApprovePSBT, data);
    });

    let onclick_import = {
        Callback::from(move |_: MouseEvent| {
            navigator.push(&ImportWalletRoute::ImportWalletHome);
        })
    };

    let onclick_reveal = {
        let revealed_secret = revealed_secret.clone();
        let popup_visible = popup_visible.clone();
        Callback::from(move |_: MouseEvent| {
            if revealed_secret.is_empty() {
                popup_visible.set(true);
            } else {
                revealed_secret.set(String::default());
            }
        })
    };

    let onsave = {
        let popup_visible = popup_visible.clone();
        let revealed_secret = revealed_secret.clone();
        let derivation = derivation.clone();
        Callback::from(move |password: String| {
            let mut storage = UserStorage::read(LocalStorage::default());
            let secret_str = storage.get_wallet_mut(&selected_wallet).and_then(|w| {
                w.reveal_secret(&password)
                    .ok()
                    .map(|s| (s, w.derivation.to_string()))
            });

            match secret_str {
                Some((s, d)) => {
                    revealed_secret.set(s);
                    derivation.set(d);
                }
                None => revealed_secret.set("No secret found".to_string()),
            }

            popup_visible.set(false);
        })
    };

    let oncancel = {
        let popup_visible = popup_visible.clone();
        Callback::from(move |_| {
            popup_visible.set(false);
        })
    };

    let mut secret_data = html! {};
    if !revealed_secret.is_empty() {
        let revealed_secret = (*revealed_secret).clone();
        let derivation = (*derivation).clone();
        secret_data = html! {
            <>
                <hr />
                <label>{"Secret:"}</label>
                <textarea disabled={true} value={revealed_secret}/>
                <hr />
                <label>{"Derivation:"}</label>
                <input disabled={true} value={derivation}/>
            </>
        }
    }

    let reveal_message = if revealed_secret.is_empty() {
        "Reveal Secret"
    } else {
        "Hide Secret"
    };

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
            {secret_data}
            <button onclick={onclick_reveal}>{reveal_message}</button>
            <button onclick={onclick_import}>{"Import another wallet"}</button>
            <InputPasswordModal
                password_for={PasswordFor::RevalSecret}
                visible={*popup_visible}
                onsave={onsave}
                oncancel={oncancel}
            />
        </>
    }
}
