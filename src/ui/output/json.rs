use std::{cell::OnceCell, sync::Arc};

use leptos::prelude::*;

use crate::ui::ace;

#[component]
pub(super) fn Json(output: Signal<String>) -> impl IntoView {
    let editor_element = NodeRef::<leptos::html::Pre>::new();
    let editor: Arc<OnceCell<ace::Editor>> = Arc::new(OnceCell::new());
    let (editor_loaded, set_editor_loaded) = signal(());

    let get_editor = editor.clone();
    let is_dark_preferred = leptos_use::use_preferred_dark();
    let _update_editor_theme = Effect::new(move || {
        editor_loaded.get(); // react to init event
        let Some(editor) = get_editor.get() else {return};
        match is_dark_preferred.get() {
            true => editor.set_theme("ace/theme/ayu-dark"),
            false => editor.set_theme("ace/theme/ayu-light")
        }
    });

    let get_editor = editor.clone();
    let _update_editor_value = Effect::new(move || {
        editor_loaded.get(); // react to init event
        let Some(editor) = get_editor.get() else {return};
        let value = output.get();
        leptos::logging::log!("some");
        editor.set_value(&value, None);
        editor.text_input().reset_selection();
    });

    let set_editor = editor.clone();
    editor_element.on_load(move |_| {
        let editor = ace::edit("json-output");
        let options = ace::EditorOptions {
            auto_scroll_editor_into_view: Some(true),
            mode: Some("ace/mode/json".into()),
            font_size: Some(16),
            read_only: Some(true),
            highlight_active_line: Some(false),
            ..Default::default()
        };
        editor.set_options(&options);
        set_editor.set(editor).unwrap();
        set_editor_loaded.set(());
    });

    view! { 
        <pre id="json-output" style="height: 400px" node_ref=editor_element></pre>
     }
}
