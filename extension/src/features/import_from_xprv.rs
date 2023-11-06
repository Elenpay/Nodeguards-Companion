use crate::components::text_input::TextInput;
use crate::components::textarea::TextArea;
use crate::features::input_password_modal::InputPasswordModal;
use crate::switch::Route;
use crate::utils::state::PasswordFor;
use crate::utils::storage::LocalStorage;
use anyhow::Result;
use signer::storage::UserStorage;
use signer::wallet::Wallet;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(ImportFromXprv)]
pub fn import_from_xprv() -> Html {
    let navigator = use_navigator().unwrap();
    let xprv = use_state(String::default);
    let derivation = use_state(String::default);
    let wallet_name = use_state(String::default);
    let error = use_state(String::default);
    let popup_visible = use_state(|| false);
    let xprv_value = (*xprv).clone();
    let derivation_value = (*derivation).clone();
    let wallet_name_value = (*wallet_name).clone();
    let error_value = (*error).clone();

    let onclick = {
        let xprv = xprv_value.clone();
        let derivation = derivation_value.clone();
        let wallet_name = wallet_name_value.clone();
        let error = error.clone();
        let popup_visible = popup_visible.clone();
        Callback::from(move |_: MouseEvent| {
            let storage = UserStorage::read(LocalStorage::default());
            if wallet_name.is_empty() {
                error.set("Wallet name is mandatory".into());
            }

            if storage.get_wallet_ref(&wallet_name).is_some() {
                error.set("There is already a wallet with that name".into());
                return;
            }
            match Wallet::validate(&xprv, &derivation) {
                Ok(_) => {}
                Err(e) => {
                    error.set(format!("{e}"));
                    return;
                }
            }

            popup_visible.set(true);
        })
    };

    let on_change_name = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| wallet_name.set(v));
    });

    let on_change_xprv = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| xprv.set(v));
    });

    let on_change_derivation = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| derivation.set(v));
    });

    let onclick_goback = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| navigator.back())
    };

    let onsave = {
        let wallet_name_value = wallet_name_value.clone();
        let xprv = xprv_value.clone();
        let derivation = derivation_value.clone();
        let popup_visible = popup_visible.clone();
        Callback::from(move |password: String| {
            let mut storage = UserStorage::read(LocalStorage::default());
            let mut wallet = Wallet::default();

            if wallet_name_value.is_empty() {
                error.set("Wallet name is mandatory".into());
                return;
            }

            let parsed = wallet.from_xprv_str(&wallet_name_value, &xprv, &derivation, &password);

            if parsed.is_err() {
                error.set("Error while parsing secret".to_string());
            }

            storage.wallets.push(wallet);
            let stored = storage.save();

            if stored.is_err() {
                error.set("Error while storing wallet".to_string());
            } else {
                navigator.push(&Route::Home);
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

    html! {
        <>
            <h class="title">{"Import from Seed"}</h>
            <TextInput disabled={*popup_visible} value={wallet_name_value} onchange={on_change_name} placeholder="Input your wallet's name" />
            <TextArea disabled={*popup_visible} value={xprv_value} onchange={on_change_xprv} placeholder="tprv/vprv..."/>
            <TextInput disabled={*popup_visible} value={derivation_value}  onchange={on_change_derivation} placeholder="m/n'/n'..."/>
            <div class="error">{error_value}</div>
            <div class="button-bar">
                <button disabled={*popup_visible} class="cancel" onclick={onclick_goback}>{"Go back"}</button>
                <button disabled={*popup_visible} {onclick}>{"Save"}</button>
            </div>
            <InputPasswordModal
                password_for={PasswordFor::ImportingSecret}
                visible={*popup_visible}
                onsave={onsave}
                oncancel={oncancel}
            />
        </>
    }
}
