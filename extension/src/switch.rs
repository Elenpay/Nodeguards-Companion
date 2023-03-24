use crate::features::{
    approve_psbt::ApprovePSBT, create_account::CreateAccount, generate_mnemonic::GenerateMnemonic,
    home::Home, import_from_mnemonic::ImportFromMnemonic, import_wallet::ImportWallet,
};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    str::FromStr,
};
use yew::{function_component, html, Html};
use yew_router::{prelude::use_navigator, Routable};

#[derive(Clone, PartialEq)]
pub enum Mnemonic {
    Import,
    Generate,
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Mnemonic::Import => write!(f, "import"),
            Mnemonic::Generate => write!(f, "generate"),
        }
    }
}

impl FromStr for Mnemonic {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "import" => Ok(Mnemonic::Import),
            "generate" => Ok(Mnemonic::Generate),
            _ => Err("Variant not found".into()),
        }
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/home")]
    Home,
    #[at("/createaccount")]
    CreateAccount,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/mnemonic/:action")]
    Mnemonic { action: Mnemonic },
    #[at("/importwallet")]
    ImportWallet,
    #[at("/approve")]
    ApprovePSBT,
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
        Route::ImportWallet => html! { <ImportWallet /> },
        Route::Mnemonic {
            action: Mnemonic::Import,
        } => html! { <ImportFromMnemonic /> },
        Route::Mnemonic {
            action: Mnemonic::Generate,
        } => html! { <GenerateMnemonic /> },
        Route::NotFound => html! { <Redirect/> },
        Route::ApprovePSBT => html! { <ApprovePSBT/> },
    };

    html! {
        {render_route}
    }
}
