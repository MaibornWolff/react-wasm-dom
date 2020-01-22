use crate::{constants::*, html::HTMLElement};

use js_sys::{JsString, Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};

pub fn add_style_to_attributes(
    value: JsValue,
    attr_name: JsString,
    element: &mut HTMLElement,
) -> Result<(), JsValue> {
    EMPTY.with(|empty| {
        SEMICOLON.with(|semicolon| {
            let attr_value = value.unchecked_into::<js_sys::Object>();
            let attr_value = Object::keys(&attr_value)
                .iter()
                .map(|key| {
                    let value = Reflect::get(&attr_value, &key).unwrap();
                    [key, value]
                })
                .filter(|[_, v]| !v.is_null())
                .map(map_style_to_css)
                .collect::<js_sys::Array>()
                .join(semicolon);
            if &attr_value != empty {
                Reflect::set(&element.attributes, &attr_name, &attr_value)?;
            }
            Ok(())
        })
    })
}

fn map_style_to_css([key, value]: [JsValue; 2]) -> JsString {
    EMPTY.with(|empty| {
        CSS_VARIABLE.with(|css_variable| {
            COLON.with(|colon| {
                PX.with(|px| {
                    let mut css_prop: JsString = key.unchecked_into();
                    let is_custom_css_prop = css_prop.starts_with(css_variable, 0);
                    css_prop = if is_custom_css_prop {
                        css_prop
                    } else {
                        hyphenate_style_name(css_prop)
                    };
                    if let Some(css_val) = value.dyn_ref::<JsString>() {
                        let css_val = css_val.trim();
                        css_prop.concat(&colon).concat(&css_val)
                    } else if let Ok(css_val) = value.dyn_into::<js_sys::Number>() {
                        let css_val: f64 = css_val.into();
                        let suffix =
                            if css_val == 0. || is_unitless(&css_prop) || is_custom_css_prop {
                                empty.clone()
                            } else {
                                px.clone()
                            };
                        css_prop
                            .concat(&colon)
                            .concat(&css_val.into())
                            .concat(&suffix)
                    } else {
                        empty.clone()
                    }
                })
            })
        })
    })
}

fn hyphenate_style_name(css_prop: JsString) -> JsString {
    UPPER_CASE.with(|upper_case| {
        UPPER_CASE_REGEXP.with(|upper_case_pattern| {
            MS.with(|ms| {
                MS_REGEXP.with(|ms_pattern| {
                    css_prop
                        .replace_by_pattern(upper_case_pattern, upper_case)
                        .to_lower_case()
                        .replace_by_pattern(ms_pattern, ms)
                })
            })
        })
    })
}

fn is_unitless(css_prop: &JsString) -> bool {
    UNITLESS.with(|unitless| unitless.includes(css_prop, 0))
}
