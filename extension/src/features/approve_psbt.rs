use crate::{
    components::{
        psbt_summary::psbt_summary,
        select::{Select, SelectItem},
    },
    context::UserContext,
    paste_psbt,
    switch::Route,
    utils::{events::State, storage::LocalStorage},
    OperationRequestData,
};
use anyhow::anyhow;
use signer::{
    psbt_details::ApprovalScreen,
    signer::decode_psbt_and_sign,
    storage::{SettingsStorage, UserStorage},
};
use std::{cell::RefCell, rc::Rc};
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
    let storage = Rc::new(RefCell::new(UserStorage::read(LocalStorage::default())));
    let default_wallet = storage.borrow().get_default_wallet();
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

    let psbt = operation_data.psbt.clone().unwrap();

    // Everything shown to the user is derived from the PSBT itself. If it
    // cannot be decoded there is nothing trustworthy to display, so refuse
    // to offer a Sign button at all (NGC-01).
    let network = SettingsStorage::read(LocalStorage::default()).get_network();
    let psbt_details = match ApprovalScreen::for_psbt(&psbt, network) {
        ApprovalScreen::Approve(details) => details,
        ApprovalScreen::Refuse { reason } => {
            let onclick_goback = Callback::from(move |_: MouseEvent| navigator.back());
            return html! {
                <>
                    <h class="title">{"Approve PSBT"}</h>
                    <div class="error">{reason}</div>
                    <div class="button-bar">
                        <button class="cancel" onclick={onclick_goback}>{"Go back"}</button>
                    </div>
                </>
            };
        }
    };

    let onclick_save = {
        let navigator = navigator.clone();
        let selected_wallet_value = selected_wallet_value.clone();
        let storage = storage.clone();
        Callback::from(move |_: MouseEvent| {
            if password.is_empty() {
                return;
            }
            let settings_storage = SettingsStorage::read(LocalStorage::default());

            let result = storage
                .borrow_mut()
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

    let items: Vec<SelectItem> = storage
        .borrow()
        .wallets
        .iter()
        .map(|w| SelectItem::new(&w.name, &w.name))
        .collect();
    html! {
        <>
            <h class="title">{"Approve PSBT"}</h>
            {psbt_summary(&psbt_details)}
            <Select {onchange} items={items} default={selected_wallet_value}/>
            <div class="error">{error_value}</div>
            <div class="button-bar">
                <button class="cancel" onclick={onclick_goback}>{"Go back"}</button>
                <button disabled={disabled} onclick={onclick_save}>{"Sign"}</button>
            </div>
        </>
    }
}
