use anyhow::Result;
use signer::storage::UserStorage;
use web_sys::MouseEvent;
use yew::{function_component, html, use_context, use_effect_with_deps, use_state, Callback, Html};

use crate::{
    components::text_input::TextInput,
    context::{ContextAction, UserContext},
    utils::{helpers::focus, state::PasswordFor, storage::LocalStorage},
};

#[function_component(InputPasswordModal)]
pub fn input_password_modal() -> Html {
    let context = use_context::<UserContext>().unwrap();
    let password = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());
    let password_value = (*password).clone();
    let error_value = (*error).clone();

    use_effect_with_deps(
        move |_| {
            let _ = focus("password-input");
        },
        context.password_for,
    );

    match context.password_for {
        Some(password_for) => {
            let onclick = {
                let password = password_value.clone();
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

                    context.dispatch(ContextAction::InputPassword {
                        password: password.clone(),
                    });
                })
            };

            let on_change = Callback::from(move |value: Result<String>| {
                let _ = value.map(|v| password.set(v));
            });

            let button_label = match password_for {
                PasswordFor::ImportingMnemonic => "Import",
                PasswordFor::SigningPSBT => "Sign",
            };

            html! {
                <div class="modal">
                    <h class="title">{"Input your password to confirm"}</h>
                    <TextInput id={Some("password-input")} itype="password" onchange={on_change} value={password_value} placeholder="Input your password" />
                    <div class="error">{error_value}</div>
                    <button {onclick}>{button_label}</button>
                </div>
            }
        }
        None => html! {},
    }
}
