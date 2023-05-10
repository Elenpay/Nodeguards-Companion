use crate::{
    components::select::{Select, SelectItem},
    context::UserContext,
    switch::{ImportWalletRoute, Route},
    utils::{events::EventManager, storage::LocalStorage},
};
use signer::storage::UserStorage;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Home)]
pub fn home() -> Html {
    let password = use_context::<UserContext>()
        .unwrap()
        .password
        .clone()
        .unwrap_or_default();
    let navigator = use_navigator().unwrap();
    let storage = UserStorage::read(LocalStorage::default());
    let selected_wallet = use_state(|| storage.get_default_wallet());
    let revealed_secret = use_state(String::default);
    let derivation = use_state(String::default);
    let selected_wallet_value = (*selected_wallet).clone();

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
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&ImportWalletRoute::ImportWalletHome);
        })
    };

    let onclick_export = {
        let selected_wallet_value = selected_wallet_value.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::ExportXPUB {
                wallet_name: selected_wallet_value.clone(),
            });
        })
    };

    let onclick_reveal = {
        let revealed_secret = revealed_secret.clone();
        let derivation = derivation.clone();
        let selected_wallet = selected_wallet.clone();
        Callback::from(move |_: MouseEvent| {
            if !revealed_secret.is_empty() {
                revealed_secret.set(String::default());
                return;
            }
            if password.is_empty() {
                return;
            }
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
        })
    };

    let onchange = {
        let revealed_secret = revealed_secret.clone();
        let derivation = derivation.clone();
        Callback::from(move |value: SelectItem| {
            selected_wallet.set(value.label);
            revealed_secret.set(String::default());
            derivation.set(String::default());
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

    let items: Vec<SelectItem> = storage
        .wallets
        .iter()
        .map(|w| SelectItem::new(&w.name, &w.name))
        .collect();
    html! {
        <>
            <h class="title">{"Your Wallets"}</h>
            <Select {onchange} items={items} default={selected_wallet_value}/>
            {secret_data}
            <button onclick={onclick_reveal}>{reveal_message}</button>
            <button onclick={onclick_import}>{"Import another wallet"}</button>
            <button onclick={onclick_export}>{"Export XPUB"}</button>
        </>
    }
}
