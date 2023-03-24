use crate::{
    context::UserContextProvider,
    features::input_password_modal::InputPasswordModal,
    switch::{switch, Route},
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <BrowserRouter>
                <UserContextProvider>
                    <Switch<Route> render={switch} />
                    <InputPasswordModal />
                </UserContextProvider>
            </BrowserRouter>
        </div>
    }
}
