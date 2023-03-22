use anyhow::Result;
use signer::{wallet::Wallet, storage::UserStorage, signer::decode_psbt_and_sign};
use web_sys::MouseEvent;
use yew::{function_component, Html, html, use_context, Properties, Callback, use_state};
use yew_router::prelude::{use_navigator, use_location};

use crate::{utils::{storage::LocalStorage, macros::with_error_msg, state::PSBTWithWallet}, context::UserContext, switch::{PasswordFor, Route}, components::text_input::TextInput, pastePSBT};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub _for: PasswordFor
}

#[function_component(InputPassword)]
pub fn input_password(props: &Props) -> Html {
    let location = use_location().unwrap();
    let navigator = use_navigator().unwrap();
    let context = use_context::<UserContext>().unwrap();
    let password = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());
    let password_value= (*password).clone();
    let error_value= (*error).clone();

    let action = props._for.clone();
    let onclick = {
        let password = password_value.clone();
        Callback::from(move |_: MouseEvent| {
            let mut storage = UserStorage::read(LocalStorage::default());
            
            if password.is_empty() {
                error.set("You need to input the password".to_string()); 
                return
            }
            match storage.verify_password(password.as_bytes()) {
                Ok(true) => {},
                Ok(false) => { error.set("Incorrect password".to_string()); return },
                Err(_) => { error.set("Error while checking password".to_string()); return }
            }

            match action {
                PasswordFor::ImportingMnemonic => {
                    let mut wallet = Wallet::default();
                    let parsed = wallet.from_mnemonic_str(&context.wallet_name, &context.mnemonic, &password);
                    with_error_msg!(parsed, error.set("Error while parsing mnemonic".to_string()));
                    
                    storage.wallets.push(wallet);
                    let stored = storage.save();
                    with_error_msg!(stored, error.set("Error while storing wallet".to_string()));

                    navigator.push(&Route::Home);
                },
                PasswordFor::SigningPSBT => {
                    let state = location.state::<PSBTWithWallet>();
                    if let Some(s) = state {
                        let wallet = storage.wallets.iter_mut().find(|w| w.name == s.wallet_name).unwrap();
                        
                        
                        let signed_psbt = decode_psbt_and_sign(&s.psbt, wallet, &password).unwrap();
                        //with_error_msg!(signed_psbt, error.set("Error while signing PSBT".to_string()));

                        pastePSBT(signed_psbt.into());
                    }
                    navigator.push(&Route::Home); 
                }
            }
        })
    };

    let on_change = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| password.set(v));
    });
    
    let button_label = match props._for {
        PasswordFor::ImportingMnemonic => "Import",
        PasswordFor::SigningPSBT => "Sign"
    };

    html! {
        <>
            <h class="title">{"Input your password to confirm"}</h>  
            <TextInput itype="password" onchange={on_change} value={password_value} placeholder="Input your password" />
            <div class="error">{error_value}</div>
            <button {onclick}>{button_label}</button>
        </>
    }
}