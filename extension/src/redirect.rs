use signer::storage::UserStorage;
use yew::{function_component, Html, html};
use yew_router::prelude::use_navigator;
use crate::{utils::storage::LocalStorage, switch::Route};

#[function_component(Redirect)]
pub fn redirect() -> Html {
    let navigator = use_navigator().unwrap();
    let storage = UserStorage::read(LocalStorage::default());
    if !storage.has_password() || !storage.name.is_some() {
        navigator.push(&Route::Password);
    } else if storage.wallets.len() == 0 {
        navigator.push(&Route::ImportWallet);
    }

    html! {
        <div>{"All set up!"}</div>
    }
}