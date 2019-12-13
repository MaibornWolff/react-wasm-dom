use crate::{
    jsx::{Jsx, JsxProps},
    react::{React, ReactComponent},
};

use js_sys::{JsString, Reflect};
use typed_html::dom::{DOMTree, VNode};
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

        // let html: Option<DOMTree<String>> = None;
        let html = render_jsx_to_string(None, jsx, is_static)?;

        // TODO no unwrap
        Ok(html.unwrap().to_string())
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
    root: Option<DOMTree<String>>,
    jsx: &Jsx,
    is_static: bool,
) -> Result<Option<DOMTree<String>>, JsValue> {
    #[cfg(debug_assertions)]
    web_sys::console::log_2(&"JSX".into(), &jsx);

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
                Ok(root)
            } else {
                render_jsx_to_string(root, jsx.unchecked_ref::<Jsx>(), is_static)
            }
        }
        ReactComponent::Functional(function) => {
            let jsx: Jsx = function
                .call0(&JsValue::NULL)
                .expect("Functional Component initialization failed")
                .unchecked_into();
            render_jsx_to_string(root, &jsx, is_static)
        }
        ReactComponent::Intrinsic(intrinsic) => render_intrinsic(root, intrinsic, jsx, is_static),
    }
}

fn render_intrinsic(
    mut root: Option<DOMTree<String>>,
    intrinsic: String,
    jsx: &Jsx,
    is_static: bool,
) -> Result<Option<DOMTree<String>>, JsValue> {
    #[cfg(debug_assertions)]
    web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());

    if Reflect::get(&jsx.props(), &"hasOwnProperty".into())?.is_function() {
        if jsx.props().has_own_property(&"style".into()) {
            check_style_prop(jsx)?;
        }
    } else {
        handle_poisoned_has_own_property(jsx);
    }

    // TODO data-reactroot
    // TODO attributes
    // TODO other tags
    let mut element: DOMTree<String> = match intrinsic.as_ref() {
        "a" => {
            let mut element = html!(<a/>);
            if !is_static && root.is_none() {
                element.data_attributes.push(("reactroot", "".to_string()));
            }
            for prop in js_sys::Object::entries(&jsx.props()).values() {
                let prop: js_sys::Array = prop?.into();
                let value = prop.pop();
                let key = prop.pop();
                let attr_name: String = key.unchecked_into::<JsString>().into();
                match attr_name.as_ref() {
                    "hasOwnProperty" | "children" => {}
                    "accessKey" => {
                        if let Ok(value) = value.dyn_into::<JsString>() {
                            let attr_value: String = value.into();
                            element.attrs.accesskey = Some(attr_value);
                        }
                    }
                    _ => {
                        web_sys::console::warn_2(
                            &"Unknown attribute at `a` tag:".into(),
                            &attr_name.into(),
                        );
                    }
                }
            }
            element
        }
        "div" => {
            let mut element = html!(<div/>);
            if !is_static && root.is_none() {
                element.data_attributes.push(("reactroot", "".to_string()));
            }
            element
        }
        "img" => {
            let mut element = html!(<img/>);
            if !is_static && root.is_none() {
                element.data_attributes.push(("reactroot", "".to_string()));
            }
            element
        }
        "span" => {
            let mut element = html!(<span/>);
            if !is_static && root.is_none() {
                element.data_attributes.push(("reactroot", "".to_string()));
            }
            element
        }
        _ => {
            web_sys::console::log_2(&"unknown HTML tag:".into(), &intrinsic.into());
            unimplemented!();
        }
    };

    let props = jsx.props();
    let props = props.unchecked_ref::<JsxProps>();

    #[cfg(debug_assertions)]
    web_sys::console::log_2(&"PROPS".into(), &props);

    // web_sys::console::log_2(&"BEFORE".into(), &root.unwrap().clone().to_string().into());
    match &mut root {
        Some(root) => match &mut root.vnode() {
            VNode::Element(root) => {
                web_sys::console::log_2(&"PUSH".into(), &element.to_string().into());
                root.children.push(element.vnode());
            }
            _ => unimplemented!(),
        },
        None => {
            root = Some(element);
        }
    };
    // web_sys::console::log_2(&"AFTER".into(), &root.unwrap().clone().to_string().into());

    let mut is_first = true;

    if let Some(children) = props.children() {
        if let Some(children) = children.dyn_ref::<js_sys::Array>() {
            for child in children.values() {
                if !is_static && !is_first {
                    // TODO empty html comment
                } else {
                    is_first = false;
                }
                root = render_intrinsic_to_string(root, child?.into(), is_static);
            }
        } else {
            root = render_intrinsic_to_string(root, children, is_static);
        }
    }
    Ok(root)
}

fn check_style_prop(jsx: &Jsx) -> Result<(), JsValue> {
    let style = Reflect::get(&jsx.props(), &"style".into())?;
    if style.is_object() {
        Ok(())
    } else {
        let mut err =
            "The `style` prop expects a mapping from style properties to values, not \
             a string. For example, style={{marginRight: spacing + 'em'}} when using JSX."
                .to_string();
        jsx.add_component_stack(&mut err);
        Err(js_sys::Error::new(&err).into())
    }
}

fn handle_poisoned_has_own_property(jsx: &Jsx) {
    let mut err = "React does not recognize the `hasOwnProperty` prop".to_string();
    jsx.add_component_stack(&mut err);
    web_sys::console::error_1(&err.into());
}

fn render_intrinsic_to_string(
    mut root: Option<DOMTree<String>>,
    js_val: js_sys::Object,
    is_static: bool,
) -> Option<DOMTree<String>> {
    web_sys::console::log_1(&js_val);
    match js_val.dyn_ref::<JsString>() {
        Some(js_string) => {
            let s: String = js_string.into();
            render_text_component(&mut root, s);
        }
        None => {
            let jsx = js_val.unchecked_ref::<Jsx>();
            root = render_jsx_to_string(root, jsx, is_static).unwrap();
        }
    };
    root
}

fn render_text_component(root: &mut Option<DOMTree<String>>, s: String) {
    use typed_html::dom::Node;
    match root {
        Some(root) => match &mut root.vnode() {
            VNode::Element(root) => {
                web_sys::console::log_2(&"push".into(), &s.clone().into());
                // root.children.push(text!("dayum").vnode());
                root.children.push(VNode::Text(s.as_ref()));
                web_sys::console::log_1(&root.name.into());
                // panic!();
                // web_sys::console::log_1(&root.children.len());
            }
            _ => unimplemented!(),
        },
        None => unimplemented!(),
    };
}
