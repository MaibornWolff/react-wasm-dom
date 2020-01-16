use crate::{
    html::{HTMLElement, HTMLValue},
    jsx::{Jsx, JsxProps},
    react::{React, ReactComponent},
    renderer::css::add_style_to_attributes,
};

use js_sys::{JsString, Reflect};
use std::collections::HashMap;
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

        let html = render_jsx_to_string(None, jsx, js_sys::Object::new(), is_static, true)?;

        match html {
            Some(html) => Ok(format!("{}", html)),
            None => Ok("".to_string()),
        }
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
    parent: Option<HTMLElement>,
    jsx: &Jsx,
    mut context: js_sys::Object,
    is_static: bool,
    is_root: bool,
) -> Result<Option<HTMLElement>, JsValue> {
    #[cfg(debug_assertions)]
    web_sys::console::log_2(&"JSX".into(), &jsx);

    match jsx.get_component(&context)? {
        ReactComponent::Class(component, _context_types, child_context_types) => {
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"CLASS".into(), &component);
            let obj = component.unchecked_ref::<js_sys::Object>();
            let proto = js_sys::Object::get_prototype_of(obj);
            if proto.has_own_property(&"componentWillMount".into()) {
                component.component_will_mount();
            } else if proto.has_own_property(&"UNSAFE_componentWillMount".into()) {
                component.unsafe_component_will_mount();
            }
            let jsx = component.render();
            context = if child_context_types.is_truthy() {
                js_sys::Object::assign(&context, &component.get_child_context())
            } else {
                context
            };
            if jsx.is_null() {
                Ok(parent)
            } else {
                render_jsx_to_string(parent, jsx.unchecked_ref(), context, is_static, is_root)
            }
        }
        ReactComponent::Functional(function) => {
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"FUNCTIONAL".into(), &function);
            let jsx = function
                .call2(&JsValue::NULL, &jsx.props(), &context)
                .expect("Functional Component initialization failed");
            render_jsx_to_string(parent, jsx.unchecked_ref(), context, is_static, is_root)
        }
        ReactComponent::Intrinsic(intrinsic) => {
            #[cfg(debug_assertions)]
            web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());
            render_intrinsic(parent, intrinsic, jsx, context, is_static, is_root)
        }
    }
}

fn render_intrinsic(
    parent: Option<HTMLElement>,
    intrinsic: String,
    jsx: &Jsx,
    context: js_sys::Object,
    is_static: bool,
    is_root: bool,
) -> Result<Option<HTMLElement>, JsValue> {
    #[cfg(debug_assertions)]
    web_sys::console::log_2(&"INTRINSIC".into(), &intrinsic.clone().into());

    if Reflect::get(&jsx.props(), &"hasOwnProperty".into())?.is_function() {
        if jsx.props().has_own_property(&"style".into()) {
            check_style_prop(jsx)?;
        }
    } else {
        handle_poisoned_has_own_property(jsx);
    }

    let mut element = HTMLElement {
        tag: intrinsic,
        attributes: HashMap::new(),
        children: Vec::new(),
    };
    for prop in js_sys::Object::entries(&jsx.props()).values() {
        let prop: js_sys::Array = prop?.into();
        let value = prop.pop();
        let key = prop.pop();
        let attr_name: String = key.unchecked_into::<JsString>().into();
        match attr_name.as_ref() {
            "hasOwnProperty" | "children" => {}
            "style" => {
                add_style_to_attributes(value, attr_name, &mut element);
            }
            _ => {
                let attr_value: String = if let Some(attr_value) = value.dyn_ref::<JsString>() {
                    attr_value.into()
                } else if let Some(attr_value) = value.dyn_ref::<js_sys::Number>() {
                    attr_value.to_string(10)?.into()
                } else if let Some(attr_value) = value.dyn_ref::<js_sys::Object>() {
                    attr_value.to_string().into()
                } else {
                    web_sys::console::error_2(
                        &"attribute must either be string, number or object on intrinsic element:"
                            .into(),
                        &attr_name.into(),
                    );
                    panic!();
                };
                element.attributes.insert(attr_name, attr_value);
            }
        }
    }
    if !is_static && is_root {
        element
            .attributes
            .insert("data-reactroot".to_string(), "".to_string());
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
}

fn check_style_prop(jsx: &Jsx) -> Result<(), JsValue> {
    let style = Reflect::get(&jsx.props(), &"style".into())?;
    if style.is_object() {
        Ok(())
    } else {
        let mut err = "The `style` prop expects a mapping from style properties to values, not \
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
    mut parent: HTMLElement,
    js_val: js_sys::Object,
    context: js_sys::Object,
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
                let jsx = js_val.unchecked_ref::<Jsx>();
                parent =
                    render_jsx_to_string(Some(parent), jsx, context, is_static, is_root)?.unwrap();
            }
        },
    };
    Ok(Some(parent))
}

fn render_text(js_string: &JsString, parent: &mut HTMLElement, append_empty_comment: &mut bool) {
    let s: String = js_string.into();

    if *append_empty_comment {
        render_empty_comment(parent);
    }
    render_text_component(parent, s);
    *append_empty_comment = true;
}

fn render_empty_comment(parent: &mut HTMLElement) {
    parent.children.push(HTMLValue::Comment);
}

fn render_text_component(parent: &mut HTMLElement, s: String) {
    parent.children.push(HTMLValue::Text(s));
}
