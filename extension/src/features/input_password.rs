use anyhow::Result;
use signer::{wallet::Wallet, storage::UserStorage};
use web_sys::MouseEvent;
use yew::{function_component, Html, html, use_context, Properties, Callback, use_state};
use yew_router::prelude::use_navigator;

use crate::{utils::storage::LocalStorage, context::UserState, switch::{PasswordFor, Route}, components::text_input::TextInput};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub _for: PasswordFor
}

#[function_component(InputPassword)]
pub fn input_password(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let context = use_context::<UserState>().unwrap_or_default();
    let password = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());
    let password_value= (*password).clone();
    let error_value= (*error).clone();

    let onclick = {
        let password = password_value.clone();
        Callback::from(move |_: MouseEvent| {
            let mut storage = UserStorage::read(LocalStorage::default());
            
            match storage.verify_password(password.as_bytes()) {
                Ok(true) => {},
                Ok(false) => { error.set("Incorrect password".to_string()); return },
                Err(_) => { error.set("Error while checking password".to_string()); return }
            }

            match props._for {
                PasswordFor::ImportingMnemonic => {
                    let mut wallet = Wallet::default();
                    wallet.from_mnemonic_str(&context.wallet_name, &context.mnemonic, &password).unwrap();
                    
                    storage.wallets.push(wallet);
                    storage.save().unwrap();

                    navigator.push(&Route::Home);
                }
            }
        })
    };

    let on_change = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| password.set(v));
    });
    
    let button_label = match props._for {
        PasswordFor::ImportingMnemonic => "Import"
    };

    html! {
        <>
            <TextInput itype="password" onchange={on_change} value={password_value} />
            <button {onclick}>{button_label}</button>
            <div>{error_value}</div>
        </>
    }
}