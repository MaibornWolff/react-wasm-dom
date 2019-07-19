use wasm_bindgen::prelude::*;

use super::component::Component;
use super::jsx::{Jsx, JsxType};

use js_sys::{Array, JsString, Reflect};
use std::convert::TryInto;
use wasm_bindgen::JsCast;
use web_sys::Element;

#[wasm_bindgen]
#[allow(dead_code)]
pub fn render(jsx: &Jsx) -> Result<(), JsValue> {
    let element = render_jsx(jsx)?;
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    body.append_child(&element)?;
    web_sys::console::log_1(&element);
    Ok(())
}

fn render_jsx(jsx: &Jsx) -> Result<Element, JsValue> {
    web_sys::console::log_2(&"RENDER".into(), jsx);
    match jsx.jsx_type().try_into()? {
        JsxType::Component(constructor) => {
            let component: Component = Reflect::construct(&constructor, &Array::new())
                .expect("Component constructor failed")
                .unchecked_into();
            web_sys::console::log_2(&"CLASS COMPONENT".into(), &component);
            render_jsx(&component.render())
        }
        JsxType::Functional(function) => {
            let jsx: Jsx = function
                .call0(&JsValue::NULL)
                .expect("Functional Component initialization failed")
                .unchecked_into();
            web_sys::console::log_2(&"FUNCTIONAL COMPONENT".into(), &jsx);
            render_jsx(&jsx)
        }
        JsxType::Intrinsic(intrinsic) => {
            web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let element = document.create_element(&intrinsic)?;

            jsx.children()
                .for_each(&mut |val: JsValue, _index, _array| {
                    match val.dyn_ref::<JsString>() {
                        Some(js_string) => {
                            element
                                .insert_adjacent_html("beforeend".into(), &String::from(js_string))
                                .expect("insert_adjacent_html");
                        }
                        None => {
                            let jsx = val.unchecked_ref::<Jsx>();
                            let child_element = render_jsx(jsx).unwrap();
                            element
                                .insert_adjacent_element("beforeend".into(), &child_element)
                                .expect("insert_adjacent_element");
                        }
                    };
                });
            Ok(element)
        }
    }
}
