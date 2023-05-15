use anyhow::{anyhow, Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Eq)]
pub struct Props {
    pub id: Option<String>,
    pub value: String,
    pub label: String,
    pub name: String,
    pub checked: bool,
}

pub fn get_value_from_radio_event(event: Event) -> Result<String> {
    let event_target = event.target().context("Error accessing tareget")?;
    let target: HtmlInputElement = event_target
        .dyn_into()
        .map_err(|_| anyhow!("Error accessing input element"))?;
    Ok(target.value())
}

#[function_component(RadioButton)]
pub fn radio_button(props: &Props) -> Html {
    let Props {
        id,
        value,
        label,
        name,
        checked,
    } = props.clone();

    let for_name = name.clone();
    html! {
        <div>
            <input type="radio" {id} {name} {value} {checked}  />
            <label for={for_name}>{label}</label>
        </div>
    }
}
