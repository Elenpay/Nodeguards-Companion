use anyhow::{anyhow, Context, Result};
use js_sys::decode_uri_component;
use wasm_bindgen::JsCast;
use web_sys::{window, Clipboard, HtmlElement};

pub fn focus(element_id: &str) -> Result<()> {
    let window = window().context("Window not found")?;
    let document = window.document().context("Document not found")?;
    document
        .query_selector(&format!("#{element_id}"))
        .map_err(|_| anyhow!("Error while getting Element"))?
        .context("Element not found")?
        .dyn_ref::<HtmlElement>()
        .ok_or_else(|| anyhow!("Cast from Element to HtmlElement invalid"))?
        .focus()
        .map_err(|_| anyhow!("Error while focusing on element"))?;
    Ok(())
}

pub fn get_clipboard() -> Result<Clipboard> {
    window()
        .context("Window not found")?
        .navigator()
        .clipboard()
        .context("No clipboard found")
}

pub fn decode_url_string(string: &str) -> Result<String> {
    decode_uri_component(string)
        .map_err(|_| anyhow!("Error while decoding url string"))
        .map(|s| s.into())
}
