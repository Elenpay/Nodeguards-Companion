use crate::features::{
    approve_psbt::ApprovePSBT, create_account::CreateAccount, export_xpub::ExportXPUB,
    generate_seed::GenerateSeed, home::Home, import_from_seed::ImportFromSeed,
    import_from_xprv::ImportFromXprv, import_wallet::ImportWallet, settings::Settings,
};
use yew::{function_component, html, Html};
use yew_router::{prelude::use_navigator, Routable, Switch};

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/home")]
    Home,
    #[at("/createaccount")]
    CreateAccount,
    #[at("/import")]
    ImportWalletRoot,
    #[at("/import/*")]
    ImportWallet,
    #[at("/approve")]
    ApprovePSBT,
    #[at("/exportxpub/:wallet_name")]
    ExportXPUB { wallet_name: String },
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Eq, Copy)]
pub enum ImportWalletRoute {
    #[at("/import")]
    ImportWalletHome,
    #[at("/import/seed")]
    ImportSeed,
    #[at("/import/generatedseed")]
    GenerateSeed,
    #[at("/import/xprv")]
    ImportXPRV,
    #[not_found]
    #[at("/import/404")]
    NotFound,
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
        Route::ImportWalletRoot | Route::ImportWallet => {
            html! { <Switch<ImportWalletRoute> render={switch_import_wallets}/> }
        }
        Route::ApprovePSBT => html! { <ApprovePSBT/> },
        Route::ExportXPUB { wallet_name } => html! { <ExportXPUB wallet_name={wallet_name}/> },
        Route::Settings => html! { <Settings /> },
        Route::NotFound => html! { <Redirect /> },
    };

    html! {
        {render_route}
    }
}

fn switch_import_wallets(route: ImportWalletRoute) -> Html {
    let render_route = match route {
        ImportWalletRoute::ImportWalletHome => html! { <ImportWallet /> },
        ImportWalletRoute::ImportSeed => html! { <ImportFromSeed /> },
        ImportWalletRoute::GenerateSeed => html! { <GenerateSeed /> },
        ImportWalletRoute::ImportXPRV => html! { <ImportFromXprv /> },
        ImportWalletRoute::NotFound => html! { <Redirect /> },
    };

    html! {
        {render_route}
    }
}
