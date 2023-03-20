use std::rc::Rc;
use web_sys::console;
use yew::{Reducible, UseReducerHandle, Properties, Children, function_component, Html, use_reducer, html, ContextProvider};

pub enum ContextAction {
    AddWallet { wallet_name: String, mnemonic: String },
}

#[derive(Default, PartialEq, Clone)]
pub struct UserState {
    pub wallet_name: String,
    pub mnemonic: String,
}

impl Reducible for UserState {
    /// Reducer Action Type
    type Action = ContextAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let (wallet_name, mnemonic) = match action {
            ContextAction::AddWallet { wallet_name, mnemonic } => (wallet_name, mnemonic),
        };

        console::log_1(&format!("reducer: {}", &mnemonic).into());

        Self { wallet_name, mnemonic }.into()
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