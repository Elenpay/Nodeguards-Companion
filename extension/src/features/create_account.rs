use anyhow::Result;
use signer::storage::UserStorage;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    utils::storage::LocalStorage, 
    components::text_input::TextInput, switch::Route
};

#[function_component(CreateAccount)]
pub fn create_account() -> Html {
    let navigator = use_navigator().unwrap();
    let name = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let confirm_password = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());
    let name_value= (*name).clone();
    let password_value= (*password).clone();
    let confirm_password_value= (*confirm_password).clone();
    let error_value= (*error).clone();

    let onclick = {
        let name = name_value.clone();
        let password = password_value.clone();
        let confirm_password = confirm_password_value.clone();
        Callback::from(move |_: MouseEvent| {
            let mut storage = UserStorage::read(LocalStorage::default());

            if password != confirm_password {
                error.set("Password don't match".into());
                return;
            }
            storage.name = Some(name.to_string());
            storage.set_password(&password).unwrap();
            storage.save().unwrap();

            navigator.push(&Route::Home);
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

    html! {
        <div>
            <TextInput value={name_value} onchange={on_change_name} placeholder="Input your name"/>
            <TextInput itype="password" value={password_value} onchange={on_change_password} placeholder="Input your password"/>
            <TextInput itype="password" value={confirm_password_value} onchange={on_change_confirm_password} placeholder="Confirm your password"/>
            <button {onclick}>{"Create account"}</button>
            <div>{error_value}</div>
        </div>
    }
}