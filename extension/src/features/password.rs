use anyhow::Result;
use signer::storage::UserStorage;
use yew::prelude::*;

use crate::{
    utils::storage::LocalStorage, 
    components::text_input::TextInput
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub initial: bool
}


#[function_component(InputPassword)]
pub fn input_password(props: &Props) -> Html {
    let name = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let confirm_password = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());
    let mut name_value= (*name).clone();
    let mut password_value= (*password).clone();
    let mut confirm_password_value= (*confirm_password).clone();
    let mut error_value= (*confirm_password).clone();

    let onclick = {
        let initial = props.initial;
        let name = name_value.clone();
        let password = password_value.clone();
        let confirm_password = confirm_password_value.clone();
        Callback::from(move |_: MouseEvent| {
            let data = UserStorage::read(&LocalStorage::default());
            let mut storage = data.unwrap_or_default();

            if !initial {
                return;
            }

            if password != confirm_password {
                error.set("Password don't match".into());
                return;
            }
            storage.name = Some(name.to_string());
            storage.password = Some("".to_string()) // hash password
        })
    };

    let on_change_name = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| name.set(v));
    });
    let on_change_password = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| password.set(v));
    });
    let on_change_confirm_password = Callback::from(move |value: Result<String>| {
        let _ = value.map(|v| confirm_password.set(v));
    });

    if props.initial {
        html! {
            <div>
                <TextInput value={name_value} onchange={on_change_name} placeholder="Input your name"/>
                <TextInput itype="password" value={password_value} onchange={on_change_password} placeholder="Input your password"/>
                <TextInput itype="password" value={confirm_password_value} onchange={on_change_confirm_password} placeholder="Confirm your password"/>
                <button {onclick}>{"Create account"}</button>
                <div></div>
            </div>
        }
    } else {
        html! {
            <div>
                <input type="password" />
                <button {onclick}>{"Sign"}</button>
            </div>
        }
    }
}