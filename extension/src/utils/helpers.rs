use anyhow::{anyhow, Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{window, Clipboard, HtmlElement};

pub fn focus(element_id: &str) -> Result<()> {
    let window = window().context("Window not found")?;
    let document = window.document().context("Document not found")?;
    document
        .query_selector(&format!("#{}", element_id))
        .map_err(|_| anyhow!("Error while getting Element"))?
        .context("Element not found")?
        .dyn_ref::<HtmlElement>()
        .ok_or(anyhow!("Cast from Element to HtmlElement invalid"))?
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