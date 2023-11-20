use signer::{storage::SettingsStorage, Network};
use std::{cell::RefCell, str::FromStr};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    components::radio_button::{get_value_from_radio_event, RadioButton},
    switch::Route,
    utils::storage::LocalStorage,
};

#[function_component(Settings)]
pub fn settings() -> Html {
    let storage = RefCell::new(SettingsStorage::read(LocalStorage::default()));
    let navigator = use_navigator().unwrap();
    let network = use_state(|| storage.borrow().get_network());
    let error = use_state(String::new);
    let network_value = *network;
    let error_value = (*error).clone();

    let on_network_change = Callback::from(move |input_event: Event| {
        let value = get_value_from_radio_event(input_event).unwrap();
        network.set(Network::from_str(&value).unwrap());
    });

    let onclick_save = {
        Callback::from(move |_| {
            let mut s = storage.borrow_mut();
            s.set_network(&network_value.to_string());
            let stored = s.save();

            if stored.is_err() {
                error.set("Unable to save settings".to_string());
            } else {
                navigator.push(&Route::Home);
            }
        })
    };

    html! {
        <>
            <fieldset onchange={on_network_change} >
                <legend>{"Select a network"}</legend>
                <RadioButton id="mainnet" name="mainnet" value={Network::Bitcoin.to_string()} checked={network_value == Network::Bitcoin} label="Mainnet" />
                <RadioButton id="regtest" name="regtest" value={Network::Regtest.to_string()} checked={network_value == Network::Regtest} label="Regtest" />
                <RadioButton id="signet" name="signet" value={Network::Signet.to_string()} checked={network_value == Network::Signet} label="Signet" />
                <RadioButton id="testnet" name="testnet" value={Network::Testnet.to_string()} checked={network_value == Network::Testnet} label="Testnet" />
            </fieldset>
            <div class="error">{error_value}</div>
            <button onclick={onclick_save}>{"Save"}</button>
        </>
    }
}
