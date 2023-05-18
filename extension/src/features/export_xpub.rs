use crate::{
    components::text_input::TextInput,
    context::UserContext,
    utils::{
        helpers::{decode_url_string, get_clipboard},
        storage::LocalStorage,
    },
};
use anyhow::Result;
use signer::storage::{SettingsStorage, UserStorage};
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub wallet_name: String,
}

#[function_component(ExportXPUB)]
pub fn export_xpub(props: &Props) -> Html {
    let decoded_wallet_name = decode_url_string(&props.wallet_name).unwrap();
    let password = use_context::<UserContext>()
        .unwrap()
        .password
        .clone()
        .unwrap_or_default();
    let mut storage = UserStorage::read(LocalStorage::default());
    let wallet = storage.get_wallet_mut(&decoded_wallet_name);
    let navigator = use_navigator().unwrap();
    let revealed_xpub = use_state(String::default);
    let derivation = use_state(|| {
        wallet
            .as_ref()
            .map(|w| w.derivation.to_string())
            .unwrap_or_else(String::default)
    });
    let next_derivation = use_state(String::default);
    let error = use_state(String::default);
    let derivation_value = (*derivation).clone();
    let next_derivation_value = (*next_derivation).clone();
    let revealed_xpub_value = (*revealed_xpub).clone();
    let error_value = (*error).clone();
    let password_value_ue = password.clone();
    let next_derivation_value_ue = next_derivation_value.clone();
    let revealed_xpub_ue = revealed_xpub.clone();
    let next_derivation_value_deps = next_derivation_value.clone();
    let password_value_deps = password.clone();
    use_effect_with_deps(
        move |_| {
            if !password_value_ue.is_empty() {
                let mut storage = UserStorage::read(LocalStorage::default());
                let settings = SettingsStorage::read(LocalStorage::default());
                let wallet = storage.get_wallet_mut(&decoded_wallet_name);
                if let Some(w) = wallet {
                    let full_path = if next_derivation_value_ue.is_empty() {
                        w.derivation.to_string()
                    } else {
                        format!("{}/{}", w.derivation, next_derivation_value_ue)
                    };
                    if full_path.ends_with('/') {
                        return;
                    }
                    w.derive_xpub(&full_path, &password_value_ue, settings.get_network())
                        .as_ref()
                        .map_or_else(
                            |e| {
                                revealed_xpub_ue.set("Incorrect derivation path".to_string());
                                error.set(e.to_string());
                            },
                            |x| revealed_xpub_ue.set(x.clone()),
                        );
                } else {
                    error.set("Wallet not found".to_string());
                }
            }
        },
        [next_derivation_value_deps, password_value_deps],
    );

    let onclick_go_back = {
        Callback::from(move |_: MouseEvent| {
            navigator.back();
        })
    };

    let onclick_copy_xpub = {
        let revealed_xpub = revealed_xpub.clone();
        Callback::from(move |_: MouseEvent| {
            let _ = get_clipboard().map(|c| c.write_text(&revealed_xpub));
        })
    };

    let onchange = {
        let next_derivation_value = next_derivation_value.clone();
        Callback::from(move |value: Result<String>| {
            if value.is_err() {
                revealed_xpub.set("No secret found".to_string());
                return;
            }
            let value = value.unwrap();
            if value
                .chars()
                .any(|c| !c.is_ascii_digit() && c != '/' && c != '\'')
            {
                next_derivation.set(next_derivation_value.clone());
                return;
            };
            next_derivation.set(value);
        })
    };

    html! {
        <>
            <label>{"XPUB:"}</label>
            <textarea disabled={true} value={revealed_xpub_value}/>
            <hr />
            <label>{"Derivation:"}</label>
            <span class="textbox-with-prefix">
                {derivation_value}{"/"}
                <TextInput id={Some("derivation-input")} onchange={onchange} value={next_derivation_value} disabled={password.is_empty()}/>
            </span>
            <div class="error">{error_value}</div>
            <button onclick={onclick_copy_xpub}>{"Copy XPUB"}</button>
            <button onclick={onclick_go_back}>{"Go Back"}</button>
        </>
    }
}
