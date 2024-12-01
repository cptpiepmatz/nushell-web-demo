use layout::Navbar;
use leptos::logging::log;
use leptos::{ev::PointerEvent, html, prelude::*};
use nu_protocol::engine::Stack;
use output::Output;

mod layout;
mod output;

#[component]
pub fn App() -> impl IntoView {
    let mut engine_state = crate::nu::initial_engine_state();
    let mut stack = Stack::new();

    let (output, set_output) = signal(String::new());
    let textarea_element: NodeRef<html::Textarea> = NodeRef::new();
    let on_run = move |_| {
        let value = textarea_element.get().unwrap().value();
        match crate::nu::execute(&value, &mut engine_state, &mut stack, "stuff") {
            Ok(value) => set_output.set(format!("{value:#?}")),
            Err(e) => log!("got error: {e:?}"),
        };
    };

    view! {
        <Navbar />
        <section>
            <textarea
                class="textarea"
                placeholder="e.g. Hello world"
                node_ref=textarea_element
            ></textarea>
            <button class="button is-primary" on:click=on_run>
                Run
            </button>
            <pre>{output}</pre>
            <Output />
        </section>
    }
}
