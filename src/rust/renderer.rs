use wasm_bindgen::prelude::*;

use crate::{
    component::Component,
    jsx::{Jsx, JsxProps, JsxType},
    react::React,
};

use html5ever::{
    rcdom::{Node, NodeData, RcDom},
    serialize,
    tendril::Tendril,
    tree_builder::{self, Attribute, TreeSink},
    LocalName, QualName,
};
use js_sys::{Array, JsString, Reflect};
use std::{cell::RefCell, convert::TryInto, io::Cursor, rc::Rc, str::from_utf8};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

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

#[wasm_bindgen(js_name = renderToStaticMarkup)]
#[allow(dead_code)]
pub fn render_to_static_markup(react: &React, jsx: JsValue) -> Result<String, JsValue> {
    render_server_side(react, jsx, true)
}

#[wasm_bindgen(js_name = renderToString)]
#[allow(dead_code)]
pub fn render_to_string(react: &React, jsx: JsValue) -> Result<String, JsValue> {
    render_server_side(react, jsx, false)
}

pub fn render_server_side(react: &React, jsx: JsValue, is_static: bool) -> Result<String, JsValue> {
    if react.is_valid_element(&jsx) {
        let jsx = jsx.unchecked_ref::<Jsx>();

        let mut dom = RcDom::default();
        render_jsx_to_string(jsx, &mut dom, None, true, is_static)?;
        let mut cursor = Cursor::new(vec![]);
        serialize(&mut cursor, &dom.document, Default::default()).unwrap();

        Ok(from_utf8(&cursor.into_inner()).unwrap().to_string())
    } else {
        if jsx.is_object() {
            let obj = jsx.unchecked_into::<js_sys::Object>();
            let mut err =
                "Objects are not valid as a React child (found: object with keys {".to_string();
            js_sys::Object::keys(&obj).for_each(&mut |key, index, _| {
                if index != 0 {
                    err.push_str(", ");
                }
                let key: JsString = key.into();
                let key: String = key.into();
                err.push_str(&key);
            });
            err.push_str("})");
            Err(err.into())
        } else {
            // TODO print error
            Err("".into())
        }
    }
}

fn render_jsx_to_string(
    jsx: &Jsx,
    dom: &mut RcDom,
    node: Option<Rc<Node>>,
    is_root: bool,
    is_static: bool,
) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    web_sys::console::log_3(&"JSX".into(), &jsx, &is_root.into());

    match jsx.jsx_type().try_into()? {
        JsxType::Component(constructor) => {
            let component: Component = Reflect::construct(&constructor, &Array::of1(&jsx.props()))
                .expect("Component constructor failed")
                .unchecked_into();
            let jsx = component.render();
            if jsx.is_null() {
                Ok(())
            } else {
                render_jsx_to_string(&jsx.unchecked_into::<Jsx>(), dom, node, is_root, is_static)
            }
        }
        JsxType::Functional(function) => {
            let jsx: Jsx = function
                .call0(&JsValue::NULL)
                .expect("Functional Component initialization failed")
                .unchecked_into();
            render_jsx_to_string(&jsx, dom, node, is_root, is_static)
        }
        JsxType::Intrinsic(intrinsic) => {
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());

            let mut attributes = vec![];
            if is_root && !is_static {
                attributes.push(Attribute {
                    name: QualName::new(None, ns!(), LocalName::from("data-reactroot")),
                    value: Tendril::from("".to_string()),
                });
            }

            let element = tree_builder::create_element(
                dom,
                QualName::new(None, ns!(), LocalName::from(intrinsic)),
                attributes,
            );

            let props = jsx.props();
            let props = props.unchecked_ref::<JsxProps>();

            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"PROPS".into(), &props);

            if let Some(children) = props.children() {
                if let Some(children) = children.dyn_ref::<js_sys::Array>() {
                    children.for_each(&mut |val: JsValue, _index, _array| {
                        render_intrinsic_to_string(val.into(), element.clone(), dom, is_static);
                    });
                } else {
                    render_intrinsic_to_string(children, element.clone(), dom, is_static);
                }
            }
            match node {
                Some(node) => node.children.borrow_mut().push(element),
                None => dom.get_document().children.borrow_mut().push(element),
            };
            Ok(())
        }
    }
}

fn render_intrinsic_to_string(
    js_val: js_sys::Object,
    element: Rc<Node>,
    dom: &mut RcDom,
    is_static: bool,
) {
    match js_val.dyn_ref::<JsString>() {
        Some(js_string) => {
            let s: String = js_string.into();
            element
                .children
                .borrow_mut()
                .push(Node::new(NodeData::Text {
                    contents: RefCell::new(Tendril::from(s)),
                }));
        }
        None => {
            let jsx = js_val.unchecked_ref::<Jsx>();
            render_jsx_to_string(jsx, dom, Some(element.clone()), false, is_static).unwrap();
        }
    };
}
