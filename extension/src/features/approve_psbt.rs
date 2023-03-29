use crate::{
    features::input_password_modal::InputPasswordModal,
    paste_psbt,
    switch::Route,
    utils::{events::State, state::PasswordFor, storage::LocalStorage},
    OperationRequestData,
};
use anyhow::anyhow;
use signer::{psbt_details::PSBTDetails, signer::decode_psbt_and_sign, storage::UserStorage};
use web_sys::{window, HtmlSelectElement};
use yew::prelude::*;
use yew_router::prelude::{use_location, use_navigator};

#[function_component(ApprovePSBT)]
pub fn approve_psbt() -> Html {
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let state = location.state::<State>().unwrap();
    let storage = UserStorage::read(LocalStorage::default());
    let default_wallet = storage.get_default_wallet();
    let selected_wallet = use_state(|| default_wallet);
    let error = use_state(String::default);
    let popup_visible = use_state(|| false);
    let selected_wallet_value = (*selected_wallet).clone();
    let error_value = (*error).clone();
    let select_node_ref = use_node_ref();
    let operation_data = state.get_ref::<OperationRequestData>();

    let no_data = html! {
        <>
            <h class="title">{"Approve PSBT"}</h>
            <div>{"PSBT could not be parsed"}</div>
        </>
    };

    if let Some(data) = operation_data {
        if data.psbt.is_none() {
            return no_data;
        }
        let psbt = data.psbt.clone().unwrap();
        let onclick = {
            let popup_visible = popup_visible.clone();
            Callback::from(move |_: MouseEvent| {
                popup_visible.set(true);
            })
        };

        let onchange = {
            let select_node_ref = select_node_ref.clone();
            Callback::from(move |_: Event| {
                if let Some(target) = select_node_ref.cast::<HtmlSelectElement>() {
                    selected_wallet.set(target.value())
                }
            })
        };

        let onclick_goback = {
            let navigator = navigator.clone();
            Callback::from(move |_: MouseEvent| navigator.back())
        };

        let onsave = {
            let navigator = navigator.clone();
            let popup_visible = popup_visible.clone();
            let psbt = psbt.clone();
            let selected_wallet_value = selected_wallet_value.clone();
            Callback::from(move |password: String| {
                let mut storage = UserStorage::read(LocalStorage::default());

                let result = storage
                    .get_wallet_mut(&selected_wallet_value)
                    .ok_or(anyhow!("Wallet not found"))
                    .and_then(|wallet| decode_psbt_and_sign(&psbt, wallet, &password))
                    .map_err(|_| anyhow!("Error while signing PSBT"))
                    .and_then(|signed_psbt| paste_psbt(&signed_psbt))
                    .map_err(|_| anyhow!("Error while pasting PSBT"));

                match result {
                    Ok(_) => {
                        navigator.push(&Route::Home);
                        window().unwrap().close().unwrap();
                    }
                    Err(e) => error.set(format!("{}", e)),
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

        let psbt = PSBTDetails::from_str(&psbt.to_string());
        return html! {
            <>
                <h class="title">{"Approve PSBT"}</h>
                <div class="display-field">
                    <strong>{"Tx Id:"}</strong>
                    <span>{psbt.tx_id}</span>
                </div>
                <div class="display-field">
                    <strong>{"Operation Type:"}</strong>
                    <span>{operation_data.and_then(|data| data.request_type.clone())}</span>
                </div>
                <div class="display-field">
                    <strong>{"Amount:"}</strong>
                    <span>{operation_data.and_then(|data| data.amount.clone())}</span>
                    <span>{"BTC"}</span>
                </div>
                <div class="display-field">
                    <strong>{"Fee:"}</strong>
                    <span>{psbt.fee}</span>
                    <span>{"SATS"}</span>
                </div>
                <select name="wallets" {onchange} ref={select_node_ref}>
                {
                    storage.wallets.iter().map(|w| {
                        let name = w.name.to_string();
                        let value = w.name.to_string();
                        html! {
                            <option selected={selected_wallet_value == name} value={value}>{name}</option>
                        }
                }).collect::<Html>()
                }
                </select>
                <div class="error">{error_value}</div>
                <div class="button-bar">
                    <button class="cancel" onclick={onclick_goback}>{"Go back"}</button>
                    <button disabled={*popup_visible} {onclick}>{"Sign"}</button>
                </div>
                <InputPasswordModal
                    password_for={PasswordFor::SigningPSBT}
                    visible={*popup_visible}
                    onsave={onsave}
                    oncancel={oncancel}
                />
            </>
        };
    }

    no_data
}
