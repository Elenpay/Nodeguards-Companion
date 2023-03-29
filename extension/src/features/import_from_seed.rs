use crate::components::text_input::TextInput;
use crate::features::input_password_modal::InputPasswordModal;
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

#[function_component(ImportFromSeed)]
pub fn import_from_seed() -> Html {
    let navigator = use_navigator().unwrap();
    let seed = use_state(Vec::new);
    let wallet_name = use_state(String::default);
    let error = use_state(String::default);
    let popup_visible = use_state(|| false);
    let mut seed_value = (*seed).clone();
    let wallet_name_value = (*wallet_name).clone();
    let error_value = (*error).clone();

    let onpaste = {
        let error = error.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            let clipboard_event = e.dyn_into::<ClipboardEvent>().ok();
            let clipboard = clipboard_event
                .and_then(|e| e.clipboard_data())
                .ok_or_else(|| anyhow!("No clipboard found"));
            let result = clipboard
                .and_then(|c| {
                    c.get_data("text/plain")
                        .map_err(|_| anyhow!("Error while getting data from clipboard"))
                })
                .map(|t| {
                    t.split_whitespace()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                })
                .map(|v| seed.set(v));
            with_error_msg!(result, error.set("Error while generating seed".to_string()));
        })
    };

    let onclick = {
        let seed = seed_value.clone();
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
            if (*seed).len() != 24 {
                error.set("Seed not properly written".into());
                return;
            }

            popup_visible.set(true);
        })
    };

    let on_change = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| wallet_name.set(v));
    });

    let onclick_goback = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| navigator.back())
    };

    let onsave = {
        let wallet_name_value = wallet_name_value.clone();
        let seed = seed_value.clone();
        let popup_visible = popup_visible.clone();
        Callback::from(move |password: String| {
            let mut storage = UserStorage::read(LocalStorage::default());
            let mut wallet = Wallet::default();

            let parsed = wallet.from_seed_str(&wallet_name_value, &(*seed).join(" "), &password);

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

    seed_value.resize(24, String::default());
    html! {
        <>
            <h class="title">{"Import from Seed"}</h>
            <TextInput disabled={*popup_visible} value={wallet_name_value} onchange={on_change} placeholder="Input your wallet's name"/>
            <ol {onpaste}>
                {
                    seed_value.clone().iter().enumerate().map(|(index, word)| {
                        html!{
                            <li>
                                <input disabled={*popup_visible} key={index} value={word.to_string()}/>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ol>
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
