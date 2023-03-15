use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component]
pub fn ImportSeed() -> Html {
    let mnemonic = use_state(|| String::new());
    let clipboard = use_clipboard();
    let mnemonic_value= (*mnemonic).clone();

    let onpaste = {
        Callback::from(move |_: Event| {
            clipboard.read_text();
            let text = (*clipboard.text).clone();
            
            if text.is_some() {
                mnemonic.set(text.unwrap());
            }
        })
    };

    html! {
        <ol {onpaste}>
            {"v1"}
            {
                mnemonic_value.to_owned().split(" ").zip(0..24).map(|(word, index)| {
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