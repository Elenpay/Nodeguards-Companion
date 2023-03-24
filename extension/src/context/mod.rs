use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, Children, ContextProvider, Html, Properties, Reducible,
    UseReducerHandle,
};

use crate::utils::state::PasswordFor;

pub enum ContextAction {
    PasswordModal { password_for: PasswordFor },
    InputPassword { password: String },
    ClearPassword,
}

#[derive(Default, PartialEq, Clone)]
pub struct UserState {
    pub password: Option<String>,
    pub password_for: Option<PasswordFor>,
}

impl Reducible for UserState {
    type Action = ContextAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ContextAction::InputPassword { password } => Self {
                password: Some(password),
                ..UserState::default()
            },
            ContextAction::PasswordModal { password_for } => Self {
                password_for: Some(password_for),
                ..UserState::default()
            },
            ContextAction::ClearPassword => UserState::default(),
        }
        .into()
    }
}

pub type UserContext = UseReducerHandle<UserState>;

#[derive(Properties, Debug, PartialEq)]
pub struct UserContextProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn UserContextProvider(props: &UserContextProviderProps) -> Html {
    let reducer = use_reducer(UserState::default);

    html! {
        <ContextProvider<UserContext> context={reducer}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
}
