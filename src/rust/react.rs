use crate::component::Component;

use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type React;

    pub type ReactElement;

    #[wasm_bindgen(method, js_name = createElement)]
    pub fn create_element(
        react: &React,
        react_type: &JsValue,
        props: &JsValue,
        children: &JsValue,
    ) -> ReactElement;

    #[wasm_bindgen(method, js_name = isValidElement)]
    pub fn is_valid_element(react: &React, obj: &JsValue) -> bool;

    #[wasm_bindgen(method, getter, js_name = type)]
    pub fn get_type(react_element: &ReactElement) -> JsValue;
}

pub enum ReactComponent {
    Class(Component, JsValue, JsValue),
    Functional(ReactElement),
    Intrinsic(JsString),
    Fragment(Option<js_sys::Object>),
}

#[wasm_bindgen(module = "/src/js/escapeHtml.js")]
extern "C" {
    #[wasm_bindgen(js_name = escapeHtml)]
    pub fn escape_html(input: &JsString) -> JsString;
}
