use crate::{
    jsx::{Jsx, JsxProps},
    react::{React, ReactComponent},
};

use html5ever::{
    rcdom::{Node, NodeData, RcDom},
    serialize,
    tendril::Tendril,
    tree_builder::{self, Attribute, TreeSink},
    LocalName, QualName,
};
use js_sys::{JsString, Reflect};
use std::{cell::RefCell, io::Cursor, rc::Rc, str::from_utf8};
use wasm_bindgen::{prelude::*, JsCast};

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

    match jsx.get_component()? {
        ReactComponent::Class(component) => {
            let obj = component.unchecked_ref::<js_sys::Object>();
            let proto = js_sys::Object::get_prototype_of(obj);
            if proto.has_own_property(&"componentWillMount".into()) {
                component.component_will_mount();
            } else if proto.has_own_property(&"UNSAFE_componentWillMount".into()) {
                component.unsafe_component_will_mount();
            }
            let jsx = component.render();
            if jsx.is_null() {
                Ok(())
            } else {
                render_jsx_to_string(jsx.unchecked_ref::<Jsx>(), dom, node, is_root, is_static)
            }
        }
        ReactComponent::Functional(function) => {
            let jsx: Jsx = function
                .call0(&JsValue::NULL)
                .expect("Functional Component initialization failed")
                .unchecked_into();
            render_jsx_to_string(&jsx, dom, node, is_root, is_static)
        }
        ReactComponent::Intrinsic(intrinsic) => {
            render_intrinsic(intrinsic, jsx, dom, node, is_root, is_static)
        }
    }
}

fn render_intrinsic(
    intrinsic: String,
    jsx: &Jsx,
    dom: &mut RcDom,
    node: Option<Rc<Node>>,
    is_root: bool,
    is_static: bool,
) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());

    if jsx.props().has_own_property(&"style".into()) {
        check_style_prop(jsx)?;
    }

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
            for child in children.values() {
                render_intrinsic_to_string(child?.into(), element.clone(), dom, is_static);
            }
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

fn check_style_prop(jsx: &Jsx) -> Result<(), JsValue> {
    let style = Reflect::get(&jsx.props(), &"style".into())?;
    if style.is_object() {
        Ok(())
    } else {
        let err = "The `style` prop expects a mapping from style properties to values, not \
                   a string. For example, style={{marginRight: spacing + 'em'}} when using JSX.\n    in iframe (at **)";
        Err(js_sys::Error::new(err).into())
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
