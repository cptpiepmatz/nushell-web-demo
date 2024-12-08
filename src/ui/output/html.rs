use leptos::prelude::*;

#[component]
pub(super) fn Html(output: Signal<String>) -> impl IntoView {
    view! { <div class="content hide-html-output-background" inner_html=output /> }
}
