use std::{str::FromStr, error::Error, fmt::{Display, Formatter}};
use yew::{Html, html, function_component};
use yew_router::{Routable, prelude::use_navigator};
use crate::{features::{
    home::Home, create_account::CreateAccount, import_from_mnemonic::ImportFromMnemonic, import_wallet::ImportWallet, input_password::InputPassword
}, context::UserContextProvider};


#[derive(Debug, PartialEq, Clone)]
pub enum PasswordFor {
    ImportingMnemonic
}

impl Display for PasswordFor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            PasswordFor::ImportingMnemonic => write!(f, "ImportingMnemonic"),
        }
    }
}

impl FromStr for PasswordFor {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ImportingMnemonic" => Ok(PasswordFor::ImportingMnemonic),
            _ => Err("Variant not found".into())
        }
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/home")]
    Home,
    #[at("/password/:_for")]
    Password { _for: PasswordFor },
    #[at("/createaccount")]
    CreateAccount,
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
    let render_route = match routes {
        Route::Home => html! { <Home /> },
        Route::CreateAccount => html! { <CreateAccount /> },
        Route::Password { _for } => html! { <InputPassword _for={_for}/> },
        Route::ImportWallet => html! { <ImportWallet /> },
        Route::Mnemonic => html! { <ImportFromMnemonic /> },
        Route::NotFound => html! { <Redirect/> },
    };

    html! {
        <UserContextProvider>
            {render_route}
        </UserContextProvider>
    }
}