use anyhow::{Result, Context, anyhow};
use wasm_bindgen::{JsCast};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub onchange: Callback<Result<String>>,
    pub itype: Option<String>,
    pub placeholder: Option<String>,
}

fn get_value_from_input_event(e: InputEvent) -> Result<String> {
    let event: Event = e.dyn_into().map_err(|_| anyhow!("Error converting to event"))?;
    let event_target = event.target().context("Error accessing tareget")?;
    let target: HtmlInputElement = event_target.dyn_into().map_err(|_| anyhow!("Error accessing input element"))?;
    Ok(target.value())
}

/// Controlled Text Input Component
#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { value, onchange, itype, placeholder } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        onchange.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input type="text" {value} {oninput} type={itype} placeholder={placeholder} />
    }
}