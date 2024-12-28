use crate::nu::render::RenderedData;
use js_sys::wasm_bindgen::JsValue;
use layout::Navbar;
use leptos::logging::log;
use leptos::prelude::*;
use nu_protocol::engine::Stack;
use output::Output;
use std::cell::OnceCell;
use std::sync::Arc;

mod ace;
mod layout;
mod output;

#[component]
pub fn App() -> impl IntoView {
    ace::config::set("basePath", &JsValue::from("./ace"));
    ace::config::set("workerPath", &JsValue::from("./ace"));
    ace::config::set("modePath", &JsValue::from("./ace"));
    ace::config::set("themePath", &JsValue::from("./ace"));

    let mut engine_state = crate::nu::initial_engine_state();
    let mut stack = Stack::new();

    let (output, set_output) = signal(RenderedData::empty());

    let editor_element = NodeRef::<leptos::html::Pre>::new();
    let editor: Arc<OnceCell<ace::Editor>> = Arc::new(OnceCell::new());
    let (editor_loaded, set_editor_loaded) = signal(());

    let get_editor = editor.clone();
    let on_run = move |_| {
        let Some(editor) = get_editor.get() else { return };
        let value = editor.get_value();
        let value = value.trim();
        log!("on_run: {value:?}");
        match crate::nu::execute(value, &mut engine_state, &mut stack, "stuff") {
            Ok(value) => {
                set_output.set(RenderedData::render(value, &engine_state));
            }
            Err(e) => log!("got error: {e:?}"),
        };
    };

    let get_editor = editor.clone();
    let is_dark_preferred = leptos_use::use_preferred_dark();
    let _update_editor_theme = Effect::new(move || {
        editor_loaded.get();
        let Some(editor) = get_editor.get() else { return };
        match is_dark_preferred.get() {
            true => editor.set_theme("ace/theme/ayu-dark"),
            false => editor.set_theme("ace/theme/ayu-light"),
        }
    });

    let set_editor = editor.clone();
    editor_element.on_load(move |_| {
        let editor = ace::edit("editor");
        let options = ace::EditorOptions {
            auto_scroll_editor_into_view: Some(true),
            max_lines: Some(5),
            min_lines: Some(5),
            font_size: Some(16),
            ..Default::default()
        };
        editor.set_options(&options);
        set_editor.set(editor).unwrap();
        set_editor_loaded.set(());
    });

    view! {
        <Navbar />
        <section>
            <pre id="editor" node_ref=editor_element>
                version
            </pre>
            <button class="button is-primary" on:click=on_run>
                Run
            </button>
            <Output output=output />
        </section>
    }
}
