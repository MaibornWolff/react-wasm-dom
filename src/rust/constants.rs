use js_sys::{Array, JsString, RegExp};
use wasm_bindgen::JsValue;

thread_local! {
    pub static EMPTY: JsString = "".into();
    pub static REACT_ROOT: JsString = "data-reactroot".into();
    pub static HAS_OWN_PROPERTY: JsString = "hasOwnProperty".into();
    pub static STYLE: JsString = "style".into();
    pub static CHILDREN: JsString = "children".into();
    pub static OPEN_TAG: JsString = "<".into();
    pub static CLOSE_TAG: JsString = ">".into();
    pub static SELF_OPEN_TAG: JsString = "</".into();
    pub static SELF_CLOSE_TAG: JsString = "/>".into();
    pub static SPACE: JsString = " ".into();
    pub static ATTR_START: JsString = "=\"".into();
    pub static ATTR_END: JsString = "\"".into();
    pub static CSS_VARIABLE: JsString = "--".into();
    pub static COLON: JsString = ":".into();
    pub static SEMICOLON: JsString = ";".into();
    pub static PX: JsString = "px".into();
    pub static COMMENT: JsString = "<!-- -->".into();
    pub static PROTOTYPE: JsString = "prototype".into();
    pub static IS_REACT_COMPONENT: JsString = "isReactComponent".into();
    pub static COMPONENT_WILL_MOUNT: JsString = "componentWillMount".into();
    pub static UNSAFE_COMPONENT_WILL_MOUNT: JsString = "UNSAFE_componentWillMount".into();

    pub static AMPERSAND: JsString = "&amp;".into();
    pub static AMPERSAND_REGEXP: RegExp = RegExp::new("&", "g");
    pub static DOUBLE_QUOTE: JsString = "&quot;".into();
    pub static DOUBLE_QUOTE_REGEXP: RegExp = RegExp::new("\"", "g");
    pub static SINGLE_QUOTE: JsString = "&#x27;".into();
    pub static SINGLE_QUOTE_REGEXP: RegExp = RegExp::new("'", "g");
    pub static GT: JsString = "&gt;".into();
    pub static GT_REGEXP: RegExp = RegExp::new(">", "g");
    pub static LT: JsString = "&lt;".into();
    pub static LT_REGEXP: RegExp = RegExp::new("<", "g");
    pub static UPPER_CASE: JsString = "-$1".into();
    pub static UPPER_CASE_REGEXP: RegExp = RegExp::new("([A-Z])", "g");
    pub static MS: JsString = "-ms-".into();
    pub static MS_REGEXP: RegExp = RegExp::new("^ms-", "");

    pub static SELF_CLOSING: Array = [
        &JsValue::from("area"),
        &JsValue::from("base"),
        &JsValue::from("br"),
        &JsValue::from("col"),
        &JsValue::from("embed"),
        &JsValue::from("hr"),
        &JsValue::from("img"),
        &JsValue::from("input"),
        &JsValue::from("link"),
        &JsValue::from("meta"),
        &JsValue::from("param"),
        &JsValue::from("source"),
        &JsValue::from("track"),
        &JsValue::from("wbr"),
    ]
    .iter()
    .collect();

    // TODO vendor prefixes
    // let prefixes = ['Webkit', 'ms', 'Moz', 'O'];
    pub static UNITLESS: Array = [
        &JsValue::from("animationIterationCount"),
        &JsValue::from("borderImageOutset"),
        &JsValue::from("borderImageSlice"),
        &JsValue::from("borderImageWidth"),
        &JsValue::from("boxFlex"),
        &JsValue::from("boxFlexGroup"),
        &JsValue::from("boxOrdinalGroup"),
        &JsValue::from("columnCount"),
        &JsValue::from("columns"),
        &JsValue::from("flex"),
        &JsValue::from("flexGrow"),
        &JsValue::from("flexPositive"),
        &JsValue::from("flexShrink"),
        &JsValue::from("flexNegative"),
        &JsValue::from("flexOrder"),
        &JsValue::from("gridArea"),
        &JsValue::from("gridRow"),
        &JsValue::from("gridRowEnd"),
        &JsValue::from("gridRowSpan"),
        &JsValue::from("gridRowStart"),
        &JsValue::from("gridColumn"),
        &JsValue::from("gridColumnEnd"),
        &JsValue::from("gridColumnSpan"),
        &JsValue::from("gridColumnStart"),
        &JsValue::from("fontWeight"),
        &JsValue::from("lineClamp"),
        &JsValue::from("lineHeight"),
        &JsValue::from("opacity"),
        &JsValue::from("order"),
        &JsValue::from("orphans"),
        &JsValue::from("tabSize"),
        &JsValue::from("widows"),
        &JsValue::from("zIndex"),
        &JsValue::from("zoom"),
        &JsValue::from("fillOpacity"),
        &JsValue::from("floodOpacity"),
        &JsValue::from("stopOpacity"),
        &JsValue::from("strokeDasharray"),
        &JsValue::from("strokeDashoffset"),
        &JsValue::from("strokeMiterlimit"),
        &JsValue::from("strokeOpacity"),
        &JsValue::from("strokeWidth"),
    ]
    .iter()
    .collect();
}
