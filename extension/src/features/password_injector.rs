use crate::{
    context::{ContextAction, UserContext},
    get_password,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, Debug, PartialEq)]
pub struct PasswordInjectorProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn PasswordInjector(props: &PasswordInjectorProps) -> Html {
    let context = use_context::<UserContext>().unwrap();
    let existing_password = context.password.clone().unwrap_or_default();
    use_effect(move || {
        spawn_local(async move {
            match get_password().await {
                Ok(password) if !password.is_empty() && password != existing_password => {
                    context.dispatch(ContextAction::InputPassword { password })
                }
                Ok(password) if password.is_empty() => {
                    context.dispatch(ContextAction::ClearPassword)
                }
                Err(_) => context.dispatch(ContextAction::ClearPassword),
                _ => {}
            };
        });
    });

    html! {
        <>
            {props.children.clone()}
        </>
    }
}
