use crate::{
    constants::*,
    html::{HTMLElement, HTMLValue},
    jsx::{Jsx, JsxProps},
    react::{React, ReactComponent},
    react_is::ReactIs,
    renderer::css::add_style_to_attributes,
};

use js_sys::{JsString, Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(js_name = renderToStaticMarkup)]
#[allow(dead_code)]
pub fn render_to_static_markup(
    react: &React,
    react_is: &ReactIs,
    jsx: JsValue,
) -> Result<JsString, JsValue> {
    render_server_side(react, react_is, jsx, true)
}

#[wasm_bindgen(js_name = renderToString)]
#[allow(dead_code)]
pub fn render_to_string(
    react: &React,
    react_is: &ReactIs,
    jsx: JsValue,
) -> Result<JsString, JsValue> {
    render_server_side(react, react_is, jsx, false)
}

pub fn render_server_side(
    react: &React,
    react_is: &ReactIs,
    jsx: JsValue,
    is_static: bool,
) -> Result<JsString, JsValue> {
    if react.is_valid_element(&jsx) {
        let jsx = jsx.unchecked_ref::<Jsx>();

        let html = render_jsx_to_string(react_is, None, jsx, Object::new(), is_static, true)?;

        match html {
            Some(html) => Ok(html.render()),
            None => Ok("".into()),
        }
    } else {
        if jsx.is_object() {
            let obj = jsx.unchecked_into::<Object>();
            let mut err =
                "Objects are not valid as a React child (found: object with keys {".to_string();
            Object::keys(&obj).for_each(&mut |key, index, _| {
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
    react_is: &ReactIs,
    mut parent: Option<HTMLElement>,
    jsx: &Jsx,
    mut context: Object,
    is_static: bool,
    is_root: bool,
) -> Result<Option<HTMLElement>, JsValue> {
    #[cfg(debug_assertions)]
    web_sys::console::log_2(&"JSX".into(), &jsx);

    match jsx.get_component(react_is, &context)? {
        ReactComponent::Class(component, _context_types, child_context_types) => {
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"CLASS".into(), &component);
            let obj = component.unchecked_ref::<Object>();
            let proto = Object::get_prototype_of(obj);
            COMPONENT_WILL_MOUNT.with(|component_will_mount| {
                UNSAFE_COMPONENT_WILL_MOUNT.with(|unsafe_component_will_mount| {
                    if proto.has_own_property(component_will_mount) {
                        component.component_will_mount();
                    } else if proto.has_own_property(unsafe_component_will_mount) {
                        component.unsafe_component_will_mount();
                    }
                    let jsx = component.render();
                    context = if child_context_types.is_truthy() {
                        Object::assign(&context, &component.get_child_context())
                    } else {
                        context
                    };
                    if jsx.is_null() {
                        Ok(parent)
                    } else {
                        render_jsx_to_string(
                            react_is,
                            parent,
                            jsx.unchecked_ref(),
                            context,
                            is_static,
                            is_root,
                        )
                    }
                })
            })
        }
        ReactComponent::Functional(function) => {
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"FUNCTIONAL".into(), &function);
            let jsx = function
                .call2(&JsValue::NULL, &jsx.props(), &context)
                .expect("Functional Component initialization failed");
            if jsx.is_null() {
                Ok(parent)
            } else {
                render_jsx_to_string(
                    react_is,
                    parent,
                    jsx.unchecked_ref(),
                    context,
                    is_static,
                    is_root,
                )
            }
        }
        ReactComponent::Intrinsic(intrinsic) => {
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());
            render_intrinsic(
                react_is, parent, intrinsic, jsx, context, is_static, is_root,
            )
        }
        ReactComponent::Fragment(children) => {
            if let Some(children) = children {
                if let Some(children) = children.dyn_ref::<js_sys::Array>() {
                    for child in children.values() {
                        parent = render_jsx_to_string(
                            react_is,
                            parent,
                            child?.unchecked_ref(),
                            context.clone(),
                            is_static,
                            is_root,
                        )?;
                    }
                } else {
                    parent = render_jsx_to_string(
                        react_is,
                        parent,
                        children.unchecked_ref(),
                        context,
                        is_static,
                        is_root,
                    )?;
                }
            }
            Ok(parent)
        }
    }
}

fn render_intrinsic(
    react_is: &ReactIs,
    parent: Option<HTMLElement>,
    intrinsic: JsString,
    jsx: &Jsx,
    context: Object,
    is_static: bool,
    is_root: bool,
) -> Result<Option<HTMLElement>, JsValue> {
    EMPTY.with(|empty| {
        HAS_OWN_PROPERTY.with(|has_own_property| {
            STYLE.with(|style| {
                CHILDREN.with(|children| {
                    REACT_ROOT.with(|react_root| {
                        if Reflect::get(&jsx.props(), has_own_property)?.is_function() {
                            if jsx.props().has_own_property(style) {
                                check_style_prop(jsx)?;
                            }
                        } else {
                            handle_poisoned_has_own_property(jsx);
                        }
                        let mut element = HTMLElement {
                            tag: intrinsic,
                            attributes: Object::new(),
                            children: Vec::new(),
                        };
                        let props = &jsx.props();
                        for prop in Object::keys(props).values() {
                            let key = prop?;
                            let value = Reflect::get(props, &key)?;
                            let attr_name: JsString = key.unchecked_into();
                            if &attr_name == has_own_property || &attr_name == children {
                            } else if &attr_name == style {
                                add_style_to_attributes(value, attr_name, &mut element)?;
                            } else {
                                let attr_value: Option<JsString> = if let Some(attr_value) =
                                    value.dyn_ref::<JsString>()
                                {
                                    Some(attr_value.clone())
                                } else if let Some(attr_value) = value.dyn_ref::<js_sys::Number>() {
                                    Some(attr_value.to_string(10)?)
                                } else if let Some(attr_value) = value.dyn_ref::<Object>() {
                                    Some(attr_value.to_string())
                                } else {
                                    None
                                };
                                if let Some(attr_value) = attr_value {
                                    Reflect::set(&element.attributes, &attr_name, &attr_value)?;
                                }
                            }
                        }
                        if !is_static && is_root {
                            Reflect::set(&element.attributes, react_root, empty)?;
                        }
                        let props = jsx.props();
                        let props = props.unchecked_ref::<JsxProps>();
                        #[cfg(debug_assertions)]
                        web_sys::console::log_2(&"PROPS".into(), &props);
                        let mut append_empty_comment = false;
                        let mut element = Some(element);
                        if let Some(children) = props.children() {
                            if let Some(children) = children.dyn_ref::<js_sys::Array>() {
                                for child in children.values() {
                                    element = render_intrinsic_to_string(
                                        react_is,
                                        element.unwrap(),
                                        child?.into(),
                                        context.clone(),
                                        is_static,
                                        false,
                                        &mut append_empty_comment,
                                    )?;
                                }
                            } else {
                                element = render_intrinsic_to_string(
                                    react_is,
                                    element.unwrap(),
                                    children,
                                    context,
                                    is_static,
                                    false,
                                    &mut false,
                                )?;
                            }
                        }
                        if let Some(mut parent) = parent {
                            parent.children.push(HTMLValue::Element(element.unwrap()));
                            Ok(Some(parent))
                        } else {
                            Ok(element)
                        }
                    })
                })
            })
        })
    })
}

fn check_style_prop(jsx: &Jsx) -> Result<(), JsValue> {
    STYLE.with(|style| {
        let style = Reflect::get(&jsx.props(), style)?;
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
    })
}

fn handle_poisoned_has_own_property(jsx: &Jsx) {
    let mut err = "React does not recognize the `hasOwnProperty` prop".to_string();
    jsx.add_component_stack(&mut err);
    web_sys::console::error_1(&err.into());
}

fn render_intrinsic_to_string(
    react_is: &ReactIs,
    mut parent: HTMLElement,
    js_val: Object,
    context: Object,
    is_static: bool,
    is_root: bool,
    append_empty_comment: &mut bool,
) -> Result<Option<HTMLElement>, JsValue> {
    #[cfg(debug_assertions)]
    web_sys::console::log_2(&"RENDER_INTRINSIC".into(), &js_val);
    match js_val.dyn_ref::<JsString>() {
        Some(js_string) => {
            render_text(js_string, &mut parent, append_empty_comment);
        }
        None => match js_val.dyn_ref::<js_sys::Number>() {
            Some(js_number) => {
                render_text(&js_number.to_string(10)?, &mut parent, append_empty_comment);
            }
            None => {
                if js_val.is_truthy() {
                    parent = render_jsx_to_string(
                        react_is,
                        Some(parent),
                        js_val.unchecked_ref(),
                        context,
                        is_static,
                        is_root,
                    )?
                    .unwrap();
                }
            }
        },
    };
    Ok(Some(parent))
}

fn render_text(js_string: &JsString, parent: &mut HTMLElement, append_empty_comment: &mut bool) {
    if *append_empty_comment {
        render_empty_comment(parent);
    }
    render_text_component(parent, js_string.clone());
    *append_empty_comment = true;
}

fn render_empty_comment(parent: &mut HTMLElement) {
    parent.children.push(HTMLValue::Comment);
}

fn render_text_component(parent: &mut HTMLElement, s: JsString) {
    parent.children.push(HTMLValue::Text(s));
}
