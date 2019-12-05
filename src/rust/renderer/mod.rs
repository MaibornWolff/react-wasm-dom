use wasm_bindgen::prelude::*;

use crate::{
    component::Component,
    jsx::{Jsx, JsxProps, JsxType},
};

use js_sys::{Array, JsString, Reflect};
use std::convert::TryInto;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

pub mod server;

#[wasm_bindgen]
#[allow(dead_code)]
pub fn render(jsx: &Jsx) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    if let Some(element) = render_jsx(jsx, &document)? {
        let body = document.body().expect("document should have a body");
        body.append_child(&element)?;
        web_sys::console::log_1(&element);
    }

    Ok(())
}

fn render_jsx(jsx: &Jsx, document: &Document) -> Result<Option<Element>, JsValue> {
    #[cfg(debug_assertions)]
    web_sys::console::log_2(&"RENDER".into(), jsx);

    match jsx.jsx_type().try_into()? {
        JsxType::Component(constructor) => {
            let component: Component = Reflect::construct(&constructor, &Array::new())
                .expect("Component constructor failed")
                .unchecked_into();
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"CLASS COMPONENT".into(), &component);
            let jsx = component.render();
            if jsx.is_null() {
                Ok(None)
            } else {
                render_jsx(&jsx.unchecked_into::<Jsx>(), document)
            }
        }
        JsxType::Functional(function) => {
            let jsx: Jsx = function
                .call0(&JsValue::NULL)
                .expect("Functional Component initialization failed")
                .unchecked_into();
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"FUNCTIONAL COMPONENT".into(), &jsx);
            render_jsx(&jsx, document)
        }
        JsxType::Intrinsic(intrinsic) => {
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());

            let element = document.create_element(&intrinsic)?;

            let props = jsx.props();
            let props = props.unchecked_ref::<JsxProps>();

            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"PROPS".into(), &props);

            if let Some(children) = props.children() {
                if let Some(children) = children.dyn_ref::<js_sys::Array>() {
                    children.for_each(&mut |val: JsValue, _index, _array| {
                        render_intrinsic(val.into(), &element, document);
                    });
                } else {
                    render_intrinsic(children, &element, document);
                }
            }
            Ok(Some(element))
        }
    }
}

fn render_intrinsic(js_val: js_sys::Object, element: &Element, document: &Document) {
    match js_val.dyn_ref::<JsString>() {
        Some(js_string) => {
            element
                .insert_adjacent_html("beforeend".into(), &String::from(js_string))
                .expect("insert_adjacent_html");
        }
        None => {
            let jsx = js_val.unchecked_ref::<Jsx>();
            if let Ok(Some(child_element)) = render_jsx(jsx, document) {
                element
                    .insert_adjacent_element("beforeend".into(), &child_element)
                    .expect("insert_adjacent_element");
            }
        }
    };
}
