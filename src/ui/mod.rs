use crate::nu::render::RenderedData;
use codee::string::FromToStringCodec;
use js_sys::wasm_bindgen::JsValue;
use layout::Navbar;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use leptos_use::use_preferred_dark;
use nu_protocol::engine::Stack;
use output::Output;
use std::cell::OnceCell;
use std::sync::Arc;
use web_sys::UrlSearchParams;

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

    let (startup_command, set_startup_command) = signal("version".to_string());

    let (stored_command, set_stored_command, _) =
        use_local_storage::<String, FromToStringCodec>("command");
    let stored_command = stored_command.get();
    if !stored_command.trim().is_empty() {
        set_startup_command.set(stored_command);
    }

    let url_search_params = web_sys::window().unwrap().location().search().unwrap();
    let url_search_params = UrlSearchParams::new_with_str(&url_search_params).unwrap();
    if let Some(command) = url_search_params.get("command") {
        set_startup_command.set(command);
    }

    let (output, set_output) = signal(RenderedData::empty());

    let editor_element = NodeRef::<leptos::html::Pre>::new();
    let editor: Arc<OnceCell<ace::Editor>> = Arc::new(OnceCell::new());
    let (editor_loaded, set_editor_loaded) = signal(());

    let get_editor = editor.clone();
    let on_run = move |_| {
        let Some(editor) = get_editor.get() else { return };
        let value = editor.get_value();
        let value = value.trim();
        set_stored_command.set(value.to_string());
        log!("on_run: {value:?}");
        match crate::nu::execute(value, &mut engine_state, &mut stack, "stuff") {
            Ok(value) => {
                set_output.set(RenderedData::render(value, &engine_state));
            }
            Err(e) => log!("got error: {e:?}"),
        };
    };

    let get_editor = editor.clone();
    let is_dark_preferred = use_preferred_dark();
    let _update_editor_theme = Effect::new(move || {
        editor_loaded.get();
        let Some(editor) = get_editor.get() else { return };
        match is_dark_preferred.get() {
            true => editor.set_theme("ace/theme/ayu-dark"),
            false => editor.set_theme("ace/theme/ayu-light"),
        }
    });

    let _update_output_theme = Effect::new(move || match is_dark_preferred.get() {
        true => println!("jo, mach dark"),
        false => println!("mach hell"),
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
                {startup_command}
            </pre>
            <button class="button is-primary" on:click=on_run>
                Run
            </button>
            <Output output=output />
        </section>
    }
}
