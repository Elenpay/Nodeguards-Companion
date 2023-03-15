use yew::{Html, html};
use yew_router::Routable;
use crate::redirect::Redirect;
use crate::features::password::InputPassword;
use crate::features::import_from_mnemonic::ImportFromMnemonic;
use crate::features::import_wallet::ImportWallet;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/popup.html")]
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

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Redirect />},
        Route::Password => html! { <InputPassword initial={true} /> },
        Route::ImportWallet => html! { <ImportWallet /> },
        Route::Mnemonic => html! { <ImportFromMnemonic /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}