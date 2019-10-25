use js_sys::{Array, Function, Reflect};
use std::convert::TryInto;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    pub type Jsx;

    #[wasm_bindgen(method, getter)]
    pub fn props(this: &Jsx) -> js_sys::Object;

    #[wasm_bindgen(method, getter, js_name=type)]
    pub fn jsx_type(this: &Jsx) -> JsValue;

    pub type JsxProps;

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &JsxProps) -> Option<js_sys::Object>;
}

pub enum JsxType {
    Component(js_sys::Function),
    Functional(js_sys::Function),
    Intrinsic(String),
}

impl Jsx {
    fn is_constructor(function: &js_sys::Function) -> bool {
        match Reflect::construct(function, &Array::new()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl TryInto<JsxType> for JsValue {
    type Error = JsValue;

    fn try_into(self) -> Result<JsxType, Self::Error> {
        if self.is_function() {
            let function: Function = self.unchecked_into();
            if Jsx::is_constructor(&function) {
                Ok(JsxType::Component(function))
            } else {
                Ok(JsxType::Functional(function))
            }
        } else if let Some(intrinsic) = self.as_string() {
            Ok(JsxType::Intrinsic(intrinsic))
        } else {
            Err("bad jsx value".into())
        }
    }
}
