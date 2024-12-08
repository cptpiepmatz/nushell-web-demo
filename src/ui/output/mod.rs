use html::Html;
use json::Json;
use leptos::prelude::*;
use raw::Raw;

use crate::nu::render::RenderedData;

mod html;
mod json;
mod raw;

#[component]
pub fn Output(output: ReadSignal<RenderedData>) -> impl IntoView {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum OutputKind {
        Raw,
        Json,
        Html,
    }

    let table = Signal::derive(move || output.read().table.to_string());
    let json = Signal::derive(move || output.read().json.to_string());
    let html = Signal::derive(move || output.read().html.to_string());
    
    let (kind, set_kind) = signal(OutputKind::Json);

    let display_table = move || if kind.get() == OutputKind::Raw {""} else {"none"};
    let display_json = move || if kind.get() == OutputKind::Json {""} else {"none"};
    let display_html = move || if kind.get() == OutputKind::Html {""} else {"none"};

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
            <Raw style:display=display_table output=table />
            <Json style:display=display_json output=json />
            <Html style:display=display_html output=html />
        </div>
    }
}
