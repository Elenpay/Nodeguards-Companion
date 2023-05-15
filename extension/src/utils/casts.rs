use anyhow::{anyhow, Context, Result};
use js_sys::Promise;
use js_sys::{Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

pub fn get_fn(fn_name: &str) -> Result<Function> {
    let window = window().ok_or_else(|| anyhow!("Window not found"))?;
    let fn_js_value = Reflect::get(&window, &JsValue::from_str(fn_name))
        .map_err(|_| anyhow!("Error while getting JS function"))?;

    fn_js_value
        .dyn_ref::<Function>()
        .map(|f| f.to_owned())
        .ok_or_else(|| anyhow!("Cast from JS to Rust invalid"))
}

pub fn call_fn_str(fn_name: &str, arg1: &str) -> Result<()> {
    let js_function = get_fn(fn_name)?;

    js_function
        .call1(&JsValue::undefined(), &arg1.into())
        .map(|_| ())
        .map_err(|_| anyhow!("Error while calling JS function"))
}

#[allow(clippy::future_not_send)]
pub async fn call_fn_str_async(fn_name: &str, arg1: &str) -> Result<()> {
    let js_function = get_fn(fn_name)?;

    let js_promise = js_function
        .call1(&JsValue::undefined(), &arg1.into())
        .map_err(|_| anyhow!("Error while calling JS function"))?;

    JsFuture::from(Promise::from(js_promise))
        .await
        .map(|_| ())
        .map_err(|_| anyhow!("Error while converting promise to rust"))
}

#[allow(clippy::future_not_send)]
pub async fn call_fn_to_str_async(fn_name: &str) -> Result<String> {
    let js_function = get_fn(fn_name)?;

    let js_promise = js_function
        .call0(&JsValue::undefined())
        .map_err(|_| anyhow!("Error while calling JS function"))?;

    let js_value = JsFuture::from(Promise::from(js_promise))
        .await
        .map_err(|_| anyhow!("Error while converting promise to rust"))?;

    js_value
        .as_string()
        .context("Error while casting JS value to string")
}

pub fn call_fn_to_bool(fn_name: &str) -> Result<bool> {
    let js_function = get_fn(fn_name)?;

    let js_value = js_function
        .call0(&JsValue::undefined())
        .map_err(|_| anyhow!("Error while calling JS function"))?;

    js_value
        .as_bool()
        .context("Error while casting JS value to string")
}

pub fn call_fn(fn_name: &str) -> Result<()> {
    let js_function = get_fn(fn_name)?;

    js_function
        .call0(&JsValue::undefined())
        .map(|_| ())
        .map_err(|_| anyhow!("Error while calling JS function"))
}
