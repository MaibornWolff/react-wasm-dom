use crate::{component::ComponentConstructor, react::ReactComponent, react_is::ReactIs};

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
    pub fn get_component(
        &self,
        react_is: &ReactIs,
        context: &JsValue,
    ) -> Result<ReactComponent, JsValue> {
        if self.jsx_type().is_function() {
            let function: Function = self.jsx_type().unchecked_into();
            let proto = Reflect::get(&function, &"prototype".into())?;
            if !proto.is_undefined()
                && Reflect::get(&proto, &"isReactComponent".into())?.is_truthy()
            {
                self.construct(&function, context)
            } else {
                Ok(ReactComponent::Functional(function))
            }
        } else if let Some(intrinsic) = self.jsx_type().as_string() {
            Ok(ReactComponent::Intrinsic(intrinsic))
        } else if react_is.is_fragment(self) {
            Ok(ReactComponent::Fragment(
                self.props().unchecked_ref::<JsxProps>().children(),
            ))
        } else {
            Err("bad jsx value".into())
        }
    }

    fn construct(
        &self,
        function: &js_sys::Function,
        context: &JsValue,
    ) -> Result<ReactComponent, JsValue> {
        #[cfg(debug_assertions)]
        web_sys::console::log_2(&"CONTEXT".into(), context);
        let constructor: &ComponentConstructor = function.unchecked_ref();
        let component = Reflect::construct(function, &Array::of2(&self.props(), context))?;
        Ok(ReactComponent::Class(
            component.unchecked_into(),
            constructor.context_types(),
            constructor.child_context_types(),
        ))
    }

    pub fn add_component_stack(&self, err: &mut String) {
        err.push_str("\n    in ");
        let jsx_type: String = match self.jsx_type().as_string() {
            Some(t) => t,
            None => self.jsx_type().unchecked_into::<Function>().name().into(),
        };
        err.push_str(&jsx_type);
        // TODO replace **
        err.push_str(" (at **)");
    }
}
