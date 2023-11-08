use std::cell::RefCell;
use std::rc::Rc;

use crate::components::text_input::TextInput;
use crate::features::input_password_modal::InputPasswordModal;
use crate::get_password;
use crate::switch::Route;
use crate::utils::helpers::get_clipboard;
use crate::utils::macros::with_error_msg;
use crate::utils::state::PasswordFor;
use crate::utils::storage::LocalStorage;
use anyhow::Result;
use signer::storage::UserStorage;
use signer::wallet::Wallet;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

fn generate() -> Result<Vec<String>> {
    Wallet::generate_seed().map(|words| words.split_whitespace().map(ToString::to_string).collect())
}

#[function_component(GenerateSeed)]
pub fn generate_seed() -> Html {
    let navigator = use_navigator().unwrap();
    let seed = use_state(|| generate().unwrap_or_default());
    let wallet_name = use_state(String::default);
    let error = use_state(String::default);
    let popup_visible = use_state(|| false);
    let mut seed_value = (*seed).clone();
    let wallet_name_value = (*wallet_name).clone();
    let error_value = (*error).clone();
    let password = use_state(String::default);
    let storage = Rc::new(RefCell::new(UserStorage::read(LocalStorage::default())));

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let p = get_password().await;
                if let Ok(p) = p {
                    password.set(p)
                }
            })
        },
        (),
    );

    let on_click_generate = {
        let error = error.clone();
        Callback::from(move |_: MouseEvent| {
            let seed_str = generate().map(|words| seed.set(words));
            with_error_msg!(
                seed_str,
                error.set("Error while generating seed".to_string())
            );
        })
    };

    let on_click_copy = {
        let seed = seed_value.clone();
        Callback::from(move |_: MouseEvent| {
            let _ = get_clipboard().map(|c| c.write_text(&seed.join(" ")));
        })
    };

    let on_click_save = {
        let seed = seed_value.clone();
        let wallet_name = wallet_name_value.clone();
        let error = error.clone();
        let popup_visible = popup_visible.clone();
        let storage = storage.clone();
        Callback::from(move |_: MouseEvent| {
            if wallet_name.is_empty() {
                error.set("Wallet name is mandatory".into());
            }

            if storage.borrow().get_wallet_ref(&wallet_name).is_some() {
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
            let mut wallet = Wallet::default();

            if wallet_name_value.is_empty() {
                error.set("Wallet name is mandatory".into());
                return;
            }

            let parsed = wallet.from_seed_str(&wallet_name_value, &(*seed).join(" "), &password);

            if parsed.is_err() {
                error.set("Error while parsing secret".to_string());
            }

            let mut s = storage.borrow_mut();
            s.wallets.push(wallet);
            let stored = s.save();

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
            <TextInput value={wallet_name_value} onchange={on_change} placeholder="Input your wallet's name"/>
            <ol>
                {
                    seed_value.clone().iter().enumerate().map(|(index, word)| {
                        html!{
                            <li>
                                <input disabled={true} key={index} value={word.to_string()}/>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ol>
            <div class="error">{error_value}</div>
            <button disabled={*popup_visible} onclick={on_click_generate}>{"Generate Again"}</button>
            <button disabled={*popup_visible} onclick={on_click_copy}>{"Copy Seed"}</button>
            <div class="button-bar">
                <button class="cancel" onclick={onclick_goback}>{"Go back"}</button>
                <button disabled={*popup_visible} onclick={on_click_save}>{"Save"}</button>
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
