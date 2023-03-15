use anyhow::Result;
use signer::storage::{Wallet, UserStorage};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::{ClipboardEvent};
use crate::components::text_input::TextInput;
use crate::utils::storage::LocalStorage;

#[function_component(ImportFromMnemonic)]
pub fn import_from_mnemonic() -> Html {
    let mnemonic = use_state(|| Vec::new());
    let wallet_name = use_state(|| "".to_string());
    let mut mnemonic_value= (*mnemonic).clone();
    let wallet_name_value = (*wallet_name).clone();
    
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
        let mnemonic = mnemonic_value.clone();
        let wallet_name = wallet_name_value.clone();
        Callback::from(move |_: MouseEvent| {
            let data = UserStorage::read(&LocalStorage::default());
            
            let mnemonic = &*mnemonic.join(" ");
            let wallet = Wallet::from_mnemonic_str(&wallet_name, mnemonic);

            let mut storage = data.unwrap_or_default();
            storage.wallets.push(wallet);
        })
    };

    let on_change = Callback::from(move |value: Result<String>| {
            let _ = value.map(|v| wallet_name.set(v));
    });

    mnemonic_value.resize(24, "".to_string());
    html! {
        <>
            <TextInput value={wallet_name_value} onchange={on_change}/>
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
                <button {onclick}>{"Save"}</button>
            </ol>
        </>
    }
}