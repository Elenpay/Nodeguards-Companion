use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(PartialEq, Eq, Clone, Default)]
pub struct SelectItem {
    pub label: String,
    pub value: String,
}

impl SelectItem {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<SelectItem>,
    pub onchange: Callback<SelectItem, ()>,
}

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    let selected = use_state(String::default);
    let selected_value = (*selected).clone();
    let select_node_ref = use_node_ref();

    let onchange = {
        let select_node_ref = select_node_ref.clone();
        let onchange = props.onchange.clone();
        let items = props.items.clone();
        Callback::from(move |_: Event| {
            if let Some(target) = select_node_ref.cast::<HtmlSelectElement>() {
                selected.set(target.value());
                let value = target.value();
                let s = items
                    .iter()
                    .find(|el| el.label == value)
                    .cloned()
                    .unwrap_or_default();
                onchange.emit(s)
            }
        })
    };

    html! {
        <select {onchange} ref={select_node_ref}>
        {
            props.items.iter().map(|w| {
                let label = &w.label;
                let value = w.value.to_string();
                html! {
                    <option selected={&selected_value == label} value={value}>{label}</option>
                }
            }).collect::<Html>()
        }
        </select>
    }
}
