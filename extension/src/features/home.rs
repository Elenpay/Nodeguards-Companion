use signer::storage::UserStorage;
use yew::{function_component, Html, html};
use yew_router::prelude::use_navigator;
use crate::{utils::storage::LocalStorage, switch::Route};

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let storage = UserStorage::read(LocalStorage::default());
    if !storage.has_password() || !storage.name.is_some() {
        navigator.push(&Route::Password);
    } else if storage.wallets.len() == 0 {
        navigator.push(&Route::ImportWallet);
    }
    
    html! {
        <>
            <div>{"Your wallets"}</div>
            <select name="cars" id="cars">
                {
                    storage.wallets.iter().map(|w| {
                        let name = w.name.to_string();
                        let value = w.name.to_string();
                        html! {
                            <option value={value}>{name}</option>
                        }
                }).collect::<Html>()
                }
            </select>
        </>
    }
}