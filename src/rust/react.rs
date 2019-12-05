use crate::component::Component;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type React;

    #[wasm_bindgen(method, js_name = isValidElement)]
    pub fn is_valid_element(react: &React, obj: &JsValue) -> bool;
}

pub enum ReactComponent {
    Component(Component),
    Functional(js_sys::Function),
    Intrinsic(String),
}
