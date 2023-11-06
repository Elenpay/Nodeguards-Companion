use crate::{
    context::{ContextAction, UserContext},
    get_password, session_exists,
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
    let in_memory_password = context.password.clone().unwrap_or_default();
    use_effect(move || {
        if !session_exists().unwrap_or_default() {
            return;
        }
        spawn_local(async move {
            match get_password().await {
                Ok(session_password)
                    if !session_password.is_empty() && session_password != in_memory_password =>
                {
                    context.dispatch(ContextAction::InputPassword {
                        password: session_password,
                    })
                }
                Ok(session_password)
                    if session_password.is_empty() && !in_memory_password.is_empty() =>
                {
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
