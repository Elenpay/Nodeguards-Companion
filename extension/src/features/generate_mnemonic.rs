use crate::components::text_input::TextInput;
use crate::context::{ContextAction, UserContext};
use crate::switch::Route;
use crate::utils::helpers::get_clipboard;
use crate::utils::macros::with_error_msg;
use crate::utils::state::PasswordFor;
use crate::utils::storage::LocalStorage;
use anyhow::{anyhow, Result};
use signer::storage::UserStorage;
use signer::wallet::Wallet;
use wasm_bindgen::JsCast;
use web_sys::{console, window, ClipboardEvent};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

fn generate() -> Result<Vec<String>> {
    Wallet::generate_mnemonic()
        .and_then(|words| Ok(words.split_whitespace().map(|w| w.to_string()).collect()))
}

#[function_component(GenerateMnemonic)]
pub fn generate_mnemonic() -> Html {
    let context = use_context::<UserContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let mnemonic = use_state(|| generate().unwrap_or_default());
    let wallet_name = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());
    let disable_button = use_state(|| false);
    let mut mnemonic_value = (*mnemonic).clone();
    let wallet_name_value = (*wallet_name).clone();
    let error_value = (*error).clone();

    let on_click_generate = {
        let mnemonic = mnemonic.clone();
        let error = error.clone();
        Callback::from(move |_: MouseEvent| {
            let mnemonic_str = generate().and_then(|words| Ok(mnemonic.set(words)));
            with_error_msg!(
                mnemonic_str,
                error.set("Error while generating mnemonic".to_string())
            );
        })
    };

    let on_click_copy = {
        let mnemonic = mnemonic_value.clone();
        Callback::from(move |_: MouseEvent| {
            let _ = get_clipboard().and_then(|c| Ok(c.write_text(&mnemonic.join(" "))));
        })
    };

    let on_click_save = {
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
            <ol>
                {
                    mnemonic_value.to_owned().iter().enumerate().map(|(index, word)| {
                        html!{
                            <li>
                                <input disabled={true} key={index} value={word.to_string()}/>
                            </li>
                        }
                    }).collect::<Html>()
                }
                </ol>
            <div class="error">{error_value}</div>
            <button disabled={*disable_button} onclick={on_click_generate}>{"Generate Again"}</button>
            <button disabled={*disable_button} onclick={on_click_copy}>{"Copy Mnemonic"}</button>
            <button disabled={*disable_button} onclick={on_click_save}>{"Save"}</button>
        </>
    }
}
