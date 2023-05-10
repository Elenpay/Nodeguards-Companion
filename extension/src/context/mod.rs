use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, Children, ContextProvider, Html, Properties, Reducible,
    UseReducerHandle,
};

pub enum ContextAction {
    InputPassword { password: String },
    ClearPassword,
}

#[derive(Default, PartialEq, Clone, Eq)]
pub struct UserState {
    pub password: Option<String>,
}

impl Reducible for UserState {
    type Action = ContextAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ContextAction::InputPassword { password } => Self {
                password: Some(password),
            },
            ContextAction::ClearPassword => Self::default(),
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
