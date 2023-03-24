use crate::components::text_input::TextInput;
use crate::context::{ContextAction, UserContext};
use crate::switch::Route;
use crate::utils::macros::with_error_msg;
use crate::utils::state::PasswordFor;
use crate::utils::storage::LocalStorage;
use anyhow::{anyhow, Result};
use signer::storage::UserStorage;
use signer::wallet::Wallet;
use wasm_bindgen::JsCast;
use web_sys::ClipboardEvent;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(ImportFromMnemonic)]
pub fn import_from_mnemonic() -> Html {
    let context = use_context::<UserContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let mnemonic = use_state(|| Vec::new());
    let wallet_name = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());
    let disable_button = use_state(|| false);
    let mut mnemonic_value = (*mnemonic).clone();
    let wallet_name_value = (*wallet_name).clone();
    let error_value = (*error).clone();

    let onpaste = {
        let error = error.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            let clipboard_event = e.dyn_into::<ClipboardEvent>().ok();
            let clipboard = clipboard_event
                .and_then(|e| e.clipboard_data())
                .ok_or(anyhow!("No clipboard found"));
            let result = clipboard
                .and_then(|c| {
                    c.get_data("text/plain")
                        .map_err(|_| anyhow!("Error while getting data from clipboard"))
                })
                .map(|t| {
                    t.split_whitespace()
                        .map(|w| w.to_string())
                        .collect::<Vec<String>>()
                })
                .map(|v| mnemonic.set(v));
            with_error_msg!(
                result,
                error.set("Error while generating mnemonic".to_string())
            );
        })
    };

    let onclick = {
        let mnemonic = mnemonic_value.clone();
        let wallet_name = wallet_name_value.clone();
        let error = error.clone();
        let context = context.clone();
        let disable_button = disable_button.clone();
        Callback::from(move |_: MouseEvent| {
            let storage = UserStorage::read(LocalStorage::default());
            if storage.get_wallet_ref(&wallet_name).is_some() {
                error.set("There is already a wallet with that name".into());
                return;
            }
            if (*mnemonic).len() != 24 {
                error.set("Mnemonic not properly written".into());
                return;
            }

            disable_button.set(true);
            context.dispatch(ContextAction::PasswordModal {
                password_for: PasswordFor::ImportingMnemonic,
            });
        })
    };

    let on_change = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| wallet_name.set(v));
    });

    if let Some(password) = context.password.as_ref() {
        let mut storage = UserStorage::read(LocalStorage::default());
        let mut wallet = Wallet::default();
        let mnemonic = (*mnemonic_value).join(" ");
        let parsed = wallet.from_mnemonic_str(&wallet_name_value, &mnemonic, &password);

        context.dispatch(ContextAction::ClearPassword);

        if parsed.is_err() {
            error.set("Error while parsing mnemonic".to_string());
        }

        storage.wallets.push(wallet);
        let stored = storage.save();

        if stored.is_err() {
            error.set("Error while storing wallet".to_string());
        } else {
            navigator.push(&Route::Home);
        }
        disable_button.set(false);
    };

    mnemonic_value.resize(24, "".to_string());
    html! {
        <>
            <h class="title">{"Import from Mnemonic"}</h>
            <TextInput value={wallet_name_value} onchange={on_change} placeholder="Input your wallet's name"/>
            <ol {onpaste}>
                {
                    mnemonic_value.to_owned().iter().enumerate().map(|(index, word)| {
                        html!{
                            <li>
                                <input key={index} value={word.to_string()}/>
                            </li>
                        }
                    }).collect::<Html>()
                }
                </ol>
            <div class="error">{error_value}</div>
            <button disabled={*disable_button} {onclick}>{"Save"}</button>
        </>
    }
}
