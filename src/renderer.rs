use wasm_bindgen::prelude::*;

use super::component::Component;
use super::jsx::{Jsx, JsxType};

use html5ever::{
    rcdom::{Node, NodeData, RcDom},
    serialize,
    tendril::Tendril,
    tree_builder::{self, TreeSink},
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
    let element = render_jsx(jsx, &document)?;

    let body = document.body().expect("document should have a body");
    body.append_child(&element)?;

    web_sys::console::log_1(&element);
    Ok(())
}

fn render_jsx(jsx: &Jsx, document: &Document) -> Result<Element, JsValue> {
    web_sys::console::log_2(&"RENDER".into(), jsx);
    match jsx.jsx_type().try_into()? {
        JsxType::Component(constructor) => {
            let component: Component = Reflect::construct(&constructor, &Array::new())
                .expect("Component constructor failed")
                .unchecked_into();
            web_sys::console::log_2(&"CLASS COMPONENT".into(), &component);
            render_jsx(&component.render(), document)
        }
        JsxType::Functional(function) => {
            let jsx: Jsx = function
                .call0(&JsValue::NULL)
                .expect("Functional Component initialization failed")
                .unchecked_into();
            web_sys::console::log_2(&"FUNCTIONAL COMPONENT".into(), &jsx);
            render_jsx(&jsx, document)
        }
        JsxType::Intrinsic(intrinsic) => {
            web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());
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
                            let child_element = render_jsx(jsx, document).unwrap();
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

#[wasm_bindgen(js_name = renderToString)]
#[allow(dead_code)]
pub fn render_to_string(jsx: &Jsx) -> Result<String, JsValue> {
    let mut dom = RcDom::default();
    render_jsx_to_string(jsx, &mut dom, None)?;

    let mut cursor = Cursor::new(vec![]);
    serialize(&mut cursor, &dom.document, Default::default()).unwrap();

    Ok(from_utf8(&cursor.into_inner()).unwrap().to_string())
}

fn render_jsx_to_string(jsx: &Jsx, dom: &mut RcDom, node: Option<Rc<Node>>) -> Result<(), JsValue> {
    match jsx.jsx_type().try_into()? {
        JsxType::Component(constructor) => {
            let component: Component = Reflect::construct(&constructor, &Array::new())
                .expect("Component constructor failed")
                .unchecked_into();
            render_jsx_to_string(&component.render(), dom, node)
        }
        JsxType::Functional(function) => {
            let jsx: Jsx = function
                .call0(&JsValue::NULL)
                .expect("Functional Component initialization failed")
                .unchecked_into();
            render_jsx_to_string(&jsx, dom, node)
        }
        JsxType::Intrinsic(intrinsic) => {
            web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());
            let element = tree_builder::create_element(
                dom,
                QualName::new(None, ns!(), LocalName::from(intrinsic)),
                vec![],
            );

            jsx.children()
                .for_each(&mut |val: JsValue, _index, _array| {
                    match val.dyn_ref::<JsString>() {
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
                            let jsx = val.unchecked_ref::<Jsx>();
                            render_jsx_to_string(jsx, dom, Some(element.clone())).unwrap();
                        }
                    };
                });
            match node {
                Some(node) => node.children.borrow_mut().push(element),
                None => dom.get_document().children.borrow_mut().push(element),
            };
            Ok(())
        }
    }
}