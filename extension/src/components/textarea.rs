use anyhow::{anyhow, Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlTextAreaElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: Option<String>,
    pub disabled: Option<bool>,
    pub value: String,
    pub onchange: Callback<Result<String>>,
    pub itype: Option<String>,
    pub placeholder: Option<String>,
}

fn get_value_from_input_event(e: InputEvent) -> Result<String> {
    let event: Event = e
        .dyn_into()
        .map_err(|_| anyhow!("Error converting to event"))?;
    let event_target = event.target().context("Error accessing tareget")?;
    let target: HtmlTextAreaElement = event_target
        .dyn_into()
        .map_err(|_| anyhow!("Error accessing input element"))?;
    Ok(target.value())
}

#[function_component(TextArea)]
pub fn textarea(props: &Props) -> Html {
    let Props {
        value,
        onchange,
        itype,
        placeholder,
        id,
        disabled,
    } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        onchange.emit(get_value_from_input_event(input_event));
    });

    html! {
        <textarea disabled={disabled.unwrap_or_default()} {id} {value} {oninput} type={itype} placeholder={placeholder} />
    }
}
