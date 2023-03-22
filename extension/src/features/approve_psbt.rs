use signer::{psbt_details::PSBTDetails, storage::UserStorage};
use yew::prelude::*;
use yew_router::prelude::{use_location, use_navigator};
use web_sys::HtmlSelectElement;
use crate::{OperationRequestData, utils::{events::State, storage::LocalStorage, state::PSBTWithWallet}, switch::{Route, PasswordFor}};

#[function_component(ApprovePSBT)]
pub fn approve_psbt() -> Html {
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let state = location.state::<State>().unwrap();
    let storage = UserStorage::read(LocalStorage::default());
    let default_wallet = storage.wallets.first().map(|w| w.name.clone()).unwrap_or("".to_string());
    let selected_wallet = use_state(|| default_wallet);
    let selected_wallet_value = (*selected_wallet).clone();
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
            let wallet_name = selected_wallet_value.clone();
            let psbt = psbt.clone();
            Callback::from(move |_: MouseEvent| {
                navigator.push_with_state(
                    &Route::Password { _for: PasswordFor::SigningPSBT },
                    PSBTWithWallet { psbt: psbt.to_owned(), wallet_name: wallet_name.to_owned() });
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
                <button {onclick}>{"Sign"}</button>
            </>
        }
    }

    no_data
}