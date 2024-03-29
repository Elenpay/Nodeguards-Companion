use crate::{
    components::select::{Select, SelectItem},
    switch::{ImportWalletRoute, Route},
    utils::{events::EventManager, storage::LocalStorage},
};
use signer::storage::UserStorage;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Home)]
pub fn home() -> Html {
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

    let onclick_settings = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Settings);
        })
    };

    let onclick_export = {
        let selected_wallet_value = selected_wallet_value.clone();
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::ExportXPUB {
                wallet_name: selected_wallet_value.clone(),
            });
        })
    };

    let onclick_sign_psbt = {
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::ApprovePastedPSBT);
        })
    };

    let onchange = {
        let revealed_secret = revealed_secret;
        let derivation = derivation;
        Callback::from(move |value: SelectItem| {
            selected_wallet.set(value.label);
            revealed_secret.set(String::default());
            derivation.set(String::default());
        })
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
            <button onclick={onclick_import}>{"Import another wallet"}</button>
            <button onclick={onclick_export}>{"Export XPUB"}</button>
            <button onclick={onclick_sign_psbt}>{"Sign a PSBT"}</button>
            <button onclick={onclick_settings}>{"Settings"}</button>
        </>
    }
}
