use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::ClipboardEvent;

#[function_component(ImportSeed)]
pub fn import_seed() -> Html {
    let mnemonic = use_state(|| Vec::new());
    let mut mnemonic_value= (*mnemonic).clone();
    
    let onpaste = {
        Callback::from(move |e: Event| {
            let clipboard_event = e.dyn_into::<ClipboardEvent>().ok();
            let clipboard = clipboard_event.and_then(|e| e.clipboard_data());
            let _ = clipboard
                .and_then(|c| c.get_data("text/plain").ok())
                .map(|t| t.split_whitespace().map(|w| w.to_string()).collect::<Vec<String>>())
                .map(|v| mnemonic.set(v));
        })
    };

    mnemonic_value.resize(24, "".to_string());
    html! {
        <ol {onpaste}>
            {
                mnemonic_value.to_owned().iter().enumerate().map(|(index, word)| {
                    html!{
                        <li>
                            <input key={index} value={word.to_string()}/>
                        </li>
                    }
                }).collect::<Html>()
            }
            <button>{"Save"}</button>
        </ol>
    }
}