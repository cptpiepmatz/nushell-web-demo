use leptos::prelude::*;

#[component]
pub(super) fn Raw(output: Signal<String>) -> impl IntoView {
    view! {
        <pre>{output}</pre>
    }
}