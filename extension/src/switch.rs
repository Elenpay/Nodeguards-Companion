use yew::{Html, html, function_component};
use yew_router::{Routable, prelude::use_navigator};
use crate::features::{
    home::Home, password::InputPassword, import_from_mnemonic::ImportFromMnemonic, import_wallet::ImportWallet
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/home")]
    Home,
    #[at("/password")]
    Password,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/mnemonic")]
    Mnemonic,
    #[at("/importwallet")]
    ImportWallet
}

#[function_component(Redirect)]
pub fn redirect() -> Html {
    let navigator = use_navigator().unwrap();
    navigator.push(&Route::Home);

    html! {}
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Password => html! { <InputPassword initial={true} /> },
        Route::ImportWallet => html! { <ImportWallet /> },
        Route::Mnemonic => html! { <ImportFromMnemonic /> },
        Route::NotFound => html! { <Redirect/> },
    }
}