use crate::switch::{switch_main, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <BrowserRouter>
                <Switch<Route> render={switch_main} />
            </BrowserRouter>
        </div>
    }
}
