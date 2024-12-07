use leptos::prelude::*;

#[component]
pub(super) fn Json(output: Signal<String>) -> impl IntoView {
    view! {
        <json-viewer data=output></json-viewer>
    }
}