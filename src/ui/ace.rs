use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ace)]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen]
    pub type Editor;

    #[wasm_bindgen]
    pub fn edit(id: &str) -> Editor;

    #[wasm_bindgen(method, getter, js_name = textInput)]
    pub fn text_input(this: &Editor) -> TextInput;

    #[wasm_bindgen(method, js_name = setOptions)]
    fn _set_options(this: &Editor, options: &JsValue);

    #[wasm_bindgen(method, js_name = getValue)]
    pub fn get_value(this: &Editor) -> String;

    #[wasm_bindgen(method, js_name = setValue)]
    fn _set_value(this: &Editor, val: &str, cursor: Option<u32>);

    #[wasm_bindgen(method, js_name = setTheme)]
    pub fn set_theme(this: &Editor, theme: &str);
}

impl Editor {
    pub fn set_options(&self, options: &EditorOptions) {
        let options = serde_wasm_bindgen::to_value(options).unwrap();
        self._set_options(&options);
    }

    pub fn set_value(&self, val: &str, cursor: impl Into<Option<u32>>) {
        self._set_value(val, cursor.into())
    }
}

// type is roughly translated by ChatGPT to represent
// https://ace.c9.io/api/interfaces/ace.Ace.EditorOptions.html
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated_scroll: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scroll_editor_into_view: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behaviours_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_with_empty_selection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_style: Option<String>, // "ace" | "slim" | "smooth" | "wide"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_scrollbar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_indent_guides: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_delay: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_indent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_autocompletion: Option<bool>, // or Vec<Completer>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_keyboard_accessibility: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_live_autocompletion: Option<bool>, // or Vec<Completer>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_mobile_menu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_multiselect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_snippets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_fold_widgets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_line_number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_width_gutter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fold_style: Option<String>, // "markbegin" | "markbeginend" | "manual"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_scroll_bar_always_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_css_transforms: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_active_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_gutter_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_indent_guides: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_selected_word: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indented_soft_wrap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_handler: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_autocompletion_delay: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_autocompletion_threshold: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pixel_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_undo_deltas: Option<String>, // boolean | "always"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_lines: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigate_within_soft_tabs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_line_mode: Option<String>, // Ace.NewLineMode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_margin: Option<u32>, // number | boolean
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_margin_column: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_line_numbers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_past_end: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>, // Ace.EditSession
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_fold_widgets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_folded_annotations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_gutter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_invisibles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_line_numbers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_print_margin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_input_aria_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_follows_mouse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_soft_tabs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_svg_gutter_icons: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_worker: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_scroll_bar_always_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<String>, // number | boolean | "off" | "free" | "printmargin"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_behaviours_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_method: Option<String>, // "code" | "text" | "auto"
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["ace", "Ace"])]
    pub type TextInput;

    #[wasm_bindgen(method, js_name = resetSelection)]
    pub fn reset_selection(this: &TextInput);
}

pub mod config {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["ace", "config"])]
    extern "C" {
        #[wasm_bindgen]
        pub fn set(key: &str, value: &JsValue);
    }
}
