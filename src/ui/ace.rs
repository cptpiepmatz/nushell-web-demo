use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ace)]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen]
    pub type Editor;

    #[wasm_bindgen]
    pub fn edit(id: &str) -> Editor;

    #[wasm_bindgen(method, js_name = setOptions)]
    fn _set_options(this: &Editor, options: &JsValue);
}

impl Editor {
    pub fn set_options(&self, options: &Options) {
        let options = serde_wasm_bindgen::to_value(options).unwrap();
        self._set_options(&options);
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scroll_editor_into_view: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<u32>,
}
