use crate::{
    components::select::{Select, SelectItem},
    context::UserContext,
    paste_psbt,
    switch::Route,
    utils::{events::State, storage::LocalStorage},
    OperationRequestData,
};
use anyhow::anyhow;
use signer::{
    psbt_details::PSBTDetails,
    signer::decode_psbt_and_sign,
    storage::{SettingsStorage, UserStorage},
};
use std::str::FromStr;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::{use_location, use_navigator};

#[function_component(ApprovePSBT)]
pub fn approve_psbt() -> Html {
    let password = use_context::<UserContext>()
        .unwrap()
        .password
        .clone()
        .unwrap_or_default();
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let state = location.state::<State>().unwrap();
    let storage = UserStorage::read(LocalStorage::default());
    let default_wallet = storage.get_default_wallet();
    let selected_wallet = use_state(|| default_wallet);
    let error = use_state(String::default);
    let selected_wallet_value = (*selected_wallet).clone();
    let error_value = (*error).clone();
    let default_value = &OperationRequestData::default();
    let operation_data = state
        .get_ref::<OperationRequestData>()
        .unwrap_or(default_value);
    let disabled = password.is_empty();

    if operation_data.psbt.is_none() {
        return html! {
            <>
                <h class="title">{"Approve PSBT"}</h>
                <div>{"PSBT could not be parsed"}</div>
            </>
        };
    }

    let onclick_save = {
        let psbt = operation_data.psbt.clone().unwrap();
        let navigator = navigator.clone();
        let selected_wallet_value = selected_wallet_value.clone();
        Callback::from(move |_: MouseEvent| {
            if password.is_empty() {
                return;
            }
            let mut storage = UserStorage::read(LocalStorage::default());
            let settings_storage = SettingsStorage::read(LocalStorage::default());

            let result = storage
                .get_wallet_mut(&selected_wallet_value)
                .ok_or_else(|| anyhow!("Wallet not found"))
                .and_then(|wallet| {
                    decode_psbt_and_sign(&psbt, wallet, &password, settings_storage.get_network())
                        .map_err(|e| anyhow!("Error while signing PSBT {e}"))
                })
                .and_then(|signed_psbt| {
                    paste_psbt(&signed_psbt).map_err(|_| anyhow!("Error while pasting PSBT"))
                });

            match result {
                Ok(_) => {
                    navigator.push(&Route::Home);
                    window().unwrap().close().unwrap();
                }
                Err(e) => error.set(format!("{e}")),
            }
        })
    };

    let onchange = {
        Callback::from(move |value: SelectItem| {
            selected_wallet.set(value.label);
        })
    };

    let onclick_goback = { Callback::from(move |_: MouseEvent| navigator.back()) };

    let psbt = operation_data.psbt.clone().unwrap();
    let items: Vec<SelectItem> = storage
        .wallets
        .iter()
        .map(|w| SelectItem::new(&w.name, &w.name))
        .collect();
    let psbt = PSBTDetails::from_str(&psbt).unwrap_or_default();
    let data = operation_data.clone();
    html! {
        <>
            <h class="title">{"Approve PSBT"}</h>
            <div class="display-field">
                <strong>{"Tx Id:"}</strong>
                <span>{psbt.tx_id}</span>
            </div>
            <div class="display-field">
                <strong>{"Operation Type:"}</strong>
                <span>{data.request_type.clone()}</span>
            </div>
            <div class="display-field">
                <strong>{"Amount:"}</strong>
                <span>{data.amount.clone()}</span>
                <span>{"BTC"}</span>
            </div>
            <div class="display-field">
                <strong>{"Fee:"}</strong>
                <span>{psbt.fee}</span>
                <span>{"SATS"}</span>
            </div>
            <Select {onchange} items={items} default={selected_wallet_value}/>
            <div class="error">{error_value}</div>
            <div class="button-bar">
                <button class="cancel" onclick={onclick_goback}>{"Go back"}</button>
                <button disabled={disabled} onclick={onclick_save}>{"Sign"}</button>
            </div>
        </>
    }
}
