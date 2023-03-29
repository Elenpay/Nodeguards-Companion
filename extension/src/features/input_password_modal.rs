use anyhow::Result;
use signer::storage::UserStorage;
use web_sys::MouseEvent;
use yew::prelude::*;

use crate::{
    components::text_input::TextInput,
    utils::{helpers::focus, state::PasswordFor, storage::LocalStorage},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub visible: bool,
    pub password_for: PasswordFor,
    pub onsave: Callback<String, ()>,
    pub oncancel: Callback<(), ()>,
}

#[function_component(InputPasswordModal)]
pub fn input_password_modal(props: &Props) -> Html {
    let password = use_state(String::default);
    let error = use_state(String::default);
    let checkbox_state = use_state(|| false);
    let password_value = (*password).clone();
    let error_value = (*error).clone();

    let route = props.password_for;
    let visible = props.visible;
    use_effect_with_deps(
        move |_| {
            if !visible {
                return;
            }
            let _ = match route {
                PasswordFor::ImportingSecret => focus("confirm-checkbox"),
                _ => focus("password-input"),
            };
        },
        props.visible,
    );

    if !props.visible {
        return html! {};
    }

    let onclick = {
        let password = password.clone();
        let onsave = props.onsave.clone();
        Callback::from(move |_: MouseEvent| {
            let storage = UserStorage::read(LocalStorage::default());

            if password.is_empty() {
                error.set("You need to input the password".to_string());
                return;
            }
            match storage.verify_password(password.as_bytes()) {
                Ok(true) => {}
                Ok(false) => {
                    error.set("Incorrect password".to_string());
                    return;
                }
                Err(_) => {
                    error.set("Error while checking password".to_string());
                    return;
                }
            }

            onsave.emit((*password).clone());
            password.set(String::default());
        })
    };

    let on_change = {
        let password = password.clone();
        Callback::from(move |value: Result<String>| {
            let _ = value.map(|v| password.set(v));
        })
    };

    let button_label = match props.password_for {
        PasswordFor::ImportingSecret => "Import",
        PasswordFor::SigningPSBT => "Sign",
        PasswordFor::RevalSecret => "Reveal",
    };

    let onclick_cancel = {
        let oncancel = props.oncancel.clone();
        Callback::from(move |_: MouseEvent| {
            oncancel.emit(());
            password.set(String::default());
        })
    };

    let onchange_checkbox = {
        let checkbox_state = checkbox_state.clone();
        Callback::from(move |_: Event| checkbox_state.set(!*checkbox_state))
    };

    let checkbox = match props.password_for {
        PasswordFor::ImportingSecret => html! {
            <div class="checkbox-container">
                <input id="confirm-checkbox" type="checkbox" checked={*checkbox_state} onchange={onchange_checkbox} />
                <label>{r#"I have copied this seed into a safe place.
                I understand that if I remove this extension my seed will be lost forever"#}</label>
            </div>
        },
        PasswordFor::SigningPSBT | PasswordFor::RevalSecret => html! {},
    };

    let save_disabled = match props.password_for {
        PasswordFor::ImportingSecret => !*checkbox_state || password_value.is_empty(),
        PasswordFor::SigningPSBT | PasswordFor::RevalSecret => password_value.is_empty(),
    };

    html! {
        <div class="modal">
            <h class="title">{"Input your password to confirm"}</h>
            {checkbox}
            <TextInput id={Some("password-input")} itype="password" onchange={on_change} value={password_value} placeholder="Input your password" />
            <div class="error">{error_value}</div>
            <div class="button-bar">
                <button class="cancel" onclick={onclick_cancel}>{"Cancel"}</button>
                <button disabled={save_disabled} {onclick}>{button_label}</button>
            </div>
        </div>
    }
}
