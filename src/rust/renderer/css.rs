use crate::html::HTMLElement;

use js_sys::JsString;
use wasm_bindgen::{prelude::*, JsCast};

pub fn add_style_to_attributes(value: JsValue, attr_name: String, element: &mut HTMLElement) {
    let attr_value = value.unchecked_into::<js_sys::Object>();
    let attr_value = js_sys::Object::entries(&attr_value)
        .values()
        .into_iter()
        .filter_map(Result::ok)
        .map(map_style_to_css)
        .collect::<Vec<String>>()
        .join(";");
    let attr_value = format!("{}", attr_value);
    element.attributes.insert(attr_name, attr_value);
}

fn map_style_to_css(value: JsValue) -> String {
    let prop: js_sys::Array = value.into();
    let value = prop.pop();
    let key = prop.pop();
    let css_prop: String = key.unchecked_into::<JsString>().into();
    if let Some(css_val) = value.dyn_ref::<JsString>() {
        let css_val: String = css_val.into();
        format!("{}:{}", css_prop, css_val)
    } else if let Ok(css_val) = value.dyn_into::<js_sys::Number>() {
        let css_val: f64 = css_val.into();
        // TODO check for custom prop
        let suffix = if css_val == 0. || is_unitless(css_prop.as_ref()) {
            ""
        } else {
            "px"
        };
        format!("{}:{}{}", css_prop, css_val, suffix)
    } else {
        "".to_string()
    }
}

fn is_unitless(css_prop: &str) -> bool {
    // TODO vendor prefixes
    // let prefixes = ['Webkit', 'ms', 'Moz', 'O'];
    match css_prop {
        "animationIterationCount"
        | "borderImageOutset"
        | "borderImageSlice"
        | "borderImageWidth"
        | "boxFlex"
        | "boxFlexGroup"
        | "boxOrdinalGroup"
        | "columnCount"
        | "columns"
        | "flex"
        | "flexGrow"
        | "flexPositive"
        | "flexShrink"
        | "flexNegative"
        | "flexOrder"
        | "gridArea"
        | "gridRow"
        | "gridRowEnd"
        | "gridRowSpan"
        | "gridRowStart"
        | "gridColumn"
        | "gridColumnEnd"
        | "gridColumnSpan"
        | "gridColumnStart"
        | "fontWeight"
        | "lineClamp"
        | "lineHeight"
        | "opacity"
        | "order"
        | "orphans"
        | "tabSize"
        | "widows"
        | "zIndex"
        | "zoom"
        | "fillOpacity"
        | "floodOpacity"
        | "stopOpacity"
        | "strokeDasharray"
        | "strokeDashoffset"
        | "strokeMiterlimit"
        | "strokeOpacity"
        | "strokeWidth" => true,
        _ => false,
    }
}
