use crate::component::Component;

use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type React;

    #[wasm_bindgen(method, js_name = isValidElement)]
    pub fn is_valid_element(react: &React, obj: &JsValue) -> bool;
}

pub enum ReactComponent {
    Class(Component, JsValue, JsValue),
    Functional(js_sys::Function),
    Intrinsic(JsString),
    Fragment(Option<js_sys::Object>),
}

#[wasm_bindgen(module = "/src/js/escapeHtml.js")]
extern "C" {
    #[wasm_bindgen(js_name = escapeHtml)]
    pub fn escape_html(input: &JsString) -> JsString;
}
