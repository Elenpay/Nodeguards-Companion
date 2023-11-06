use std::rc::Rc;

use anyhow::Result;
use signer::storage::UserStorage;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;

use crate::{
    components::text_input::TextInput,
    context::{ContextAction, UserContext},
    save_password,
    utils::{helpers::focus, state::PasswordFor, storage::LocalStorage},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub visible: Option<bool>,
    pub password_for: PasswordFor,
    pub onsave: Option<Callback<String, ()>>,
    pub oncancel: Option<Callback<(), ()>>,
}

#[function_component(InputPasswordModal)]
pub fn input_password_modal(props: &Props) -> Html {
    let storage = UserStorage::read(LocalStorage::default());
    let context = use_context::<UserContext>().unwrap();
    let password_session = context.password.clone().unwrap_or_default();
    let password = use_state(String::default);
    let error = use_state(String::default);
    let checkbox_state = use_state(|| false);
    let error_value = (*error).clone();
    let password_value = (*password).clone();

    let route = props.password_for;
    let visible = storage.has_password()
        && (props.visible.unwrap_or_default() || password_session.is_empty());
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
        visible,
    );

    if !visible {
        return html! {};
    }

    let onclick = {
        let password = password.clone();
        let onsave = props.onsave.clone().unwrap_or_default();
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

            let p = (*password).clone();
            let context = context.clone();
            spawn_local(async move {
                let _ = save_password(&p).await;
                // force a refresh of the UI
                context.dispatch(ContextAction::InputPassword { password: p });
            });
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
        PasswordFor::UnlockingApp => "Unlock",
    };

    let onclick_cancel = {
        let oncancel = props.oncancel.clone().unwrap_or_default();
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
        PasswordFor::UnlockingApp => html! {},
    };

    let save_disabled = match props.password_for {
        PasswordFor::ImportingSecret => !*checkbox_state || password_value.is_empty(),
        PasswordFor::UnlockingApp => password_value.is_empty(),
    };

    let title = match props.password_for {
        PasswordFor::ImportingSecret => "Input your password to confirm",
        PasswordFor::UnlockingApp => "Input your password to unlock extension",
    };

    let onkeypress = {
        let onclick = Rc::new(onclick.clone());
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                let _ = MouseEvent::new("click").map(|e| {
                    let _ = &onclick.emit(e);
                });
            }
        })
    };

    html! {
        <div class="modal-backdrop">
            <div class="modal">
                <h class="title">{title}</h>
                {checkbox}
                <TextInput id={Some("password-input")} itype="password" onchange={on_change} value={password_value} {onkeypress} placeholder="Input your password" />
                <div class="error">{error_value}</div>
                <div class="button-bar">
                    <button class="cancel" onclick={onclick_cancel}>{"Cancel"}</button>
                    <button disabled={save_disabled} onclick={onclick}>{button_label}</button>
                </div>
            </div>
        </div>
    }
}
