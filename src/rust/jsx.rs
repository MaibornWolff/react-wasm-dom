use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type Jsx;

    #[wasm_bindgen(method, getter)]
    pub fn props(this: &Jsx) -> js_sys::Object;

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &Jsx) -> js_sys::Array;

    #[wasm_bindgen(method, getter, js_name=type)]
    pub fn jsx_type(this: &Jsx) -> js_sys::Function;
}

pub enum JsxType {
    Component(JsValue),
    Functional(js_sys::Function)
}
