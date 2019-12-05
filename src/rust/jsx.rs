use crate::react::ReactComponent;

use js_sys::{Array, Function, Reflect};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    pub type Jsx;

    #[wasm_bindgen(method, getter, js_name = "$$typeof")]
    pub fn type_of(this: &Jsx) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn props(this: &Jsx) -> js_sys::Object;

    #[wasm_bindgen(method, getter, js_name = type)]
    pub fn jsx_type(this: &Jsx) -> JsValue;

    pub type JsxProps;

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &JsxProps) -> Option<js_sys::Object>;
}

impl Jsx {
    pub fn get_component(&self) -> Result<ReactComponent, JsValue> {
        if self.jsx_type().is_function() {
            let function: Function = self.jsx_type().unchecked_into();
            match self.construct(&function) {
                Ok(component) => Ok(component),
                Err(_) => Ok(ReactComponent::Functional(function)),
            }
        } else if let Some(intrinsic) = self.jsx_type().as_string() {
            Ok(ReactComponent::Intrinsic(intrinsic))
        } else {
            Err("bad jsx value".into())
        }
    }

    fn construct(&self, function: &js_sys::Function) -> Result<ReactComponent, ()> {
        match Reflect::construct(function, &Array::of1(&self.props())) {
            Ok(component) => Ok(ReactComponent::Component(component.unchecked_into())),
            Err(_) => Err(()),
        }
    }
}
