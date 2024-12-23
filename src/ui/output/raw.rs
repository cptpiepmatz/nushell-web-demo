use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/ansi_up/ansi_up.js")]
extern "C" {
    // Bind the AnsiUp class
    pub type AnsiUp;

    // Bind the constructor
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnsiUp;

    // Bind the ansi_to_html method
    #[wasm_bindgen(method, js_name = ansi_to_html)]
    pub fn ansi_to_html(this: &AnsiUp, txt: &str) -> String;
}

#[component]
pub(super) fn Raw(output: Signal<String>) -> impl IntoView {
    let output = move || {
        let ansi_up = AnsiUp::new();
        ansi_up.ansi_to_html(output.get().as_str())
    };
    view! { <pre inner_html=output></pre> }
}
