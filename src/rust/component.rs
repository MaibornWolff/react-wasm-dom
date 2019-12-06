use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Component;

    #[wasm_bindgen(method, getter)]
    pub fn props(this: &Component) -> js_sys::Object;

    #[wasm_bindgen(method)]
    pub fn render(this: &Component) -> JsValue;

    #[wasm_bindgen(method, js_name = componentWillMount)]
    pub fn component_will_mount(this: &Component);

    #[wasm_bindgen(method, js_name = UNSAFE_componentWillMount)]
    pub fn unsafe_component_will_mount(this: &Component);
}
