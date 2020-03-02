use crate::{
    component::ComponentConstructor,
    constants::*,
    react::{React, ReactComponent},
    react_is::ReactIs,
};

use js_sys::{Array, Function, JsString, Reflect};
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
        react: &React,
        react_is: &ReactIs,
        updater: &JsValue,
        context: &JsValue,
    ) -> Result<ReactComponent, JsValue> {
        PROTOTYPE.with(|prototype| {
            IS_REACT_COMPONENT.with(|is_react_component| {
                let null = JsValue::NULL;
                let react_element = react.create_element(
                    &self.jsx_type(),
                    &self.props(),
                    match &self.props().unchecked_into::<JsxProps>().children() {
                        Some(children) => children,
                        None => &null,
                    },
                );
                let react_type = react_element.get_type();
                if react_type.is_function() {
                    let proto = Reflect::get(&react_type, prototype)?;
                    if !proto.is_undefined()
                        && Reflect::get(&proto, is_react_component)?.is_truthy()
                    {
                        let constructor: &ComponentConstructor = react_type.unchecked_ref();
                        let context_types = constructor.context_types();
                        let child_context_types = constructor.child_context_types();
                        let component = Reflect::construct(
                            &react_type.unchecked_into(),
                            &Array::of3(&self.props(), context, updater),
                        )?;
                        if react_is.is_element(&component) {
                            component
                                .unchecked_into::<Jsx>()
                                .get_component(react, react_is, updater, context)
                        } else {
                            Ok(ReactComponent::Class(
                                component.unchecked_into(),
                                context_types,
                                child_context_types,
                            ))
                        }
                    } else {
                        Ok(ReactComponent::Functional(react_element))
                    }
                } else if react_is.is_fragment(self) {
                    Ok(ReactComponent::Fragment(
                        self.props().unchecked_ref::<JsxProps>().children(),
                    ))
                } else if let Ok(intrinsic) = react_element.get_type().dyn_into::<JsString>() {
                    Ok(ReactComponent::Intrinsic(intrinsic))
                } else {
                    Err("bad jsx value".into())
                }
            })
        })
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
