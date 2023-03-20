use anyhow::Result;
use signer::storage::UserStorage;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::ClipboardEvent;
use yew_router::prelude::use_navigator;
use crate::components::text_input::TextInput;
use crate::context::{ContextAction, UserContext};
use crate::switch::{Route, PasswordFor};
use crate::utils::storage::LocalStorage;

#[function_component(ImportFromMnemonic)]
pub fn import_from_mnemonic() -> Html {
    let navigator = use_navigator().unwrap();
    let global_state = use_context::<UserContext>().unwrap();
    let mnemonic = use_state(|| Vec::new());
    let wallet_name = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());
    let mut mnemonic_value= (*mnemonic).clone();
    let wallet_name_value = (*wallet_name).clone();
    let error_value= (*error).clone();

    let onpaste = {
        Callback::from(move |e: Event| {
            let clipboard_event = e.dyn_into::<ClipboardEvent>().ok();
            let clipboard = clipboard_event.and_then(|e| e.clipboard_data());
            let _ = clipboard
                .and_then(|c| c.get_data("text/plain").ok())
                .map(|t| t.split_whitespace().map(|w| w.to_string()).collect::<Vec<String>>())
                .map(|v| mnemonic.set(v));
        })
    };

    let onclick = {
        let global_state = global_state.clone();
        let mnemonic = mnemonic_value.clone();
        let wallet_name = wallet_name_value.clone();
        Callback::from(move |_: MouseEvent| {
            let storage = UserStorage::read(LocalStorage::default());

            if storage.wallets.iter().find(|w| w.name.eq(&wallet_name)).is_some() {
                error.set("There is already a wallet with that name".into());
                return;
            }
            if (*mnemonic).len() != 24 {
                error.set("Mnemonic not properly written".into());
                return;
            }

            let mnemonic = (*mnemonic).join(" ");
            
            global_state.dispatch(ContextAction::AddWallet {
                wallet_name: wallet_name.to_string(),
                mnemonic,
            });

            navigator.push(&Route::Password { _for: PasswordFor::ImportingMnemonic });
        })
    };

    let on_change = Callback::from(move |value: Result<String>| {
            let _ = value.map(|v| wallet_name.set(v));
    });

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
            <button {onclick}>{"Save"}</button>
        </>
    }
}