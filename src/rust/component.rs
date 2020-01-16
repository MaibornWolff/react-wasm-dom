use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Component;

    #[wasm_bindgen(method, getter)]
    pub fn props(this: &Component) -> js_sys::Object;

    #[wasm_bindgen(method)]
    pub fn render(this: &Component) -> JsValue;

    #[wasm_bindgen(method, js_name = getChildContext)]
    pub fn get_child_context(this: &Component) -> js_sys::Object;

    #[wasm_bindgen(method, js_name = componentWillMount)]
    pub fn component_will_mount(this: &Component);

    #[wasm_bindgen(method, js_name = UNSAFE_componentWillMount)]
    pub fn unsafe_component_will_mount(this: &Component);
}

#[wasm_bindgen]
extern "C" {
    pub type ComponentConstructor;

    #[wasm_bindgen(method, getter, js_name = childContextTypes)]
    pub fn child_context_types(this: &ComponentConstructor) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = contextTypes)]
    pub fn context_types(this: &ComponentConstructor) -> JsValue;
}
