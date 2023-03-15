use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub button_label: String,
}


#[function_component]
pub fn InputPassword(props: &Props) -> Html {
    html! {
        <div>
            <input type="password" />
            <button>{&props.button_label}</button>
        </div>
    }
}