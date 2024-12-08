use crate::nu::render::RenderedData;
use js_sys::wasm_bindgen::JsValue;
use js_sys::{Function, Object, Reflect};
use layout::Navbar;
use leptos::logging::log;
use leptos::tachys::reactive_graph::bind::GetValue;
use leptos::{ev::PointerEvent, prelude::*};
use nu_protocol::engine::Stack;
use output::Output;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

mod ace;
mod layout;
mod output;

#[component]
pub fn App() -> impl IntoView {
    let mut engine_state = crate::nu::initial_engine_state();
    let mut stack = Stack::new();

    let (output, set_output) = signal(RenderedData::empty());
    let editor_element = NodeRef::<leptos::html::Pre>::new();
    let on_run = move |_| {
        let value = editor_element.get().unwrap().inner_text();
        let value = value.trim();
        log!("{value:?}");
        match crate::nu::execute(value, &mut engine_state, &mut stack, "stuff") {
            Ok(value) => {
                set_output.set(RenderedData::render(value, &engine_state));
            }
            Err(e) => log!("got error: {e:?}"),
        };
    };

    editor_element.on_load(|_| {
        let value = ace::edit("editor");
        let options = ace::Options {
            auto_scroll_editor_into_view: Some(true),
            max_lines: Some(8),
        };
        value.set_options(&options);
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
