use crate::{
    context::UserContextProvider,
    features::input_password_modal::InputPasswordModal,
    features::password_injector::PasswordInjector,
    open_options_page,
    switch::{switch, Route},
    utils::state::PasswordFor,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let onclick = {
        Callback::from(move |_: MouseEvent| {
            let _ = open_options_page();
        })
    };

    html! {
        <div class="app">
            <button class="open-in-tab" onclick={onclick}>
                <img src="share.png" />
            </button>
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
