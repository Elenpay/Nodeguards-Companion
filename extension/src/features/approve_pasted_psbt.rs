use crate::{
    components::{
        select::{Select, SelectItem},
        textarea::TextArea,
    },
    context::UserContext,
    utils::{helpers::get_clipboard, storage::LocalStorage},
};
use anyhow::{anyhow, Result};
use signer::{
    psbt_details::PSBTDetails,
    signer::decode_psbt_and_sign,
    storage::{SettingsStorage, UserStorage},
};
use std::{cell::RefCell, rc::Rc, str::FromStr};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(ApprovePastedPSBT)]
pub fn approve_pasted_psbt() -> Html {
    let password = use_context::<UserContext>()
        .unwrap()
        .password
        .clone()
        .unwrap_or_default();
    let navigator = use_navigator().unwrap();
    let storage = Rc::new(RefCell::new(UserStorage::read(LocalStorage::default())));
    let default_wallet = storage.borrow().get_default_wallet();
    let selected_wallet = use_state(|| default_wallet);
    let error = use_state(String::default);
    let selected_wallet_value = (*selected_wallet).clone();
    let error_value = (*error).clone();
    let disabled = password.is_empty();
    let psbt = use_state(String::default);
    let psbt_value = (*psbt).clone();
    let signed_psbt = use_state(String::default);
    let signed_psbt_value = (*signed_psbt).clone();

    let onchange_psbt = {
        let psbt = psbt.clone();
        Callback::from(move |value: Result<String>| {
            let _ = value.map(|v| psbt.set(v));
        })
    };

    let onclick_save = {
        let selected_wallet_value = selected_wallet_value.clone();
        let psbt = psbt.clone();
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
                });

            match result {
                Ok(p) => signed_psbt.set(p),
                Err(e) => error.set(e.to_string()),
            }
        })
    };

    let onchange = {
        Callback::from(move |value: SelectItem| {
            selected_wallet.set(value.label);
        })
    };

    let onclick_goback = { Callback::from(move |_: MouseEvent| navigator.back()) };

    let onclick_copy_psbt = {
        let signed_psbt_value = signed_psbt_value.clone();
        Callback::from(move |_: MouseEvent| {
            let _ = get_clipboard().map(|c| c.write_text(&signed_psbt_value));
        })
    };

    let copy_disabled = signed_psbt_value.is_empty();
    let items: Vec<SelectItem> = storage
        .borrow()
        .wallets
        .iter()
        .map(|w| SelectItem::new(&w.name, &w.name))
        .collect();

    let parsed_successfully = {
        let psbt_parsed = PSBTDetails::from_str(&psbt);
        match psbt_parsed {
            Ok(psbt) => html! {
                <div class="display-field">
                    <strong>{"Tx Id:"}</strong>
                    <span>{psbt.tx_id}</span>
                </div>
            },
            Err(_) => {
                html! {}
            }
        }
    };

    let signed_successfully = {
        if signed_psbt_value.is_empty() {
            html! {}
        } else {
            html! {
                <>
                    <TextArea value={signed_psbt_value} disabled={true} />
                    <button disabled={copy_disabled} onclick={onclick_copy_psbt}>{"Copy signed PSBT"}</button>
                </>
            }
        }
    };

    html! {
        <>
            <h class="title">{"Approve PSBT"}</h>
            <Select {onchange} items={items} default={selected_wallet_value}/>
            <TextArea value={psbt_value} onchange={onchange_psbt} placeholder="Paste your PSBT here"/>
            {parsed_successfully}
            <div class="error">{error_value}</div>
            {signed_successfully}
            <div class="button-bar">
                <button class="cancel" onclick={onclick_goback}>{"Go back"}</button>
                <button disabled={disabled} onclick={onclick_save}>{"Sign"}</button>
            </div>
        </>
    }
}
