use crate::{
    context::UserContextProvider,
    features::input_password_modal::InputPasswordModal,
    features::password_injector::PasswordInjector,
    switch::{switch, Route},
    utils::state::PasswordFor,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <BrowserRouter>
                <UserContextProvider>
                    <PasswordInjector>
                        <Switch<Route> render={switch} />
                        <InputPasswordModal password_for={PasswordFor::UnlockingApp}/>
                    </PasswordInjector>
                </UserContextProvider>
            </BrowserRouter>
        </div>
    }
}
