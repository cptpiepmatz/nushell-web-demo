use leptos::prelude::*;
use raw::Raw;
use json::Json;
use html::Html;

mod raw;
mod json;
mod html;

#[component]
pub fn Output() -> impl IntoView {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum OutputKind {
        Raw,
        Json,
        Html
    }

    let (kind, set_kind) = signal(OutputKind::Raw);

    view! {
        <div class="tabs is-boxed">
            <ul>
                <li class:is-active=move || kind.get() == OutputKind::Raw>
                    <a on:click=move |_| set_kind.set(OutputKind::Raw)>
                        <span>Raw</span>
                    </a>
                </li>
                <li class:is-active=move || kind.get() == OutputKind::Json>
                    <a on:click=move |_| set_kind.set(OutputKind::Json)>
                        <span>JSON</span>
                    </a>
                </li>
                <li class:is-active=move || kind.get() == OutputKind::Html>
                    <a on:click=move |_| set_kind.set(OutputKind::Html)>
                        <span>HTML</span>
                    </a>
                </li>
            </ul>
        </div>
        <div>
            <Show when=move || kind.get() == OutputKind::Raw><Raw /></Show>
            <Show when=move || kind.get() == OutputKind::Json><Json /></Show>
            <Show when=move || kind.get() == OutputKind::Html><Html /></Show>
        </div>
    }
}
