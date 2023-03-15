use signer::storage::UserStorage;
use yew::{function_component, Html, html};
use yew_router::prelude::use_navigator;
use crate::{utils::storage::LocalStorage, switch::Route};

#[function_component(Redirect)]
pub fn redirect() -> Html {
    let navigator = use_navigator().unwrap();
    let data = UserStorage::read(&LocalStorage::default());
    let storage = data.unwrap_or_default();
    if !storage.password.is_some() || !storage.name.is_some() {
        navigator.push(&Route::Password);
    } else if storage.wallets.len() == 0 {
        navigator.push(&Route::ImportWallet);
    }

    html! {}
}