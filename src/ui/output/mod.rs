use html::Html;
use json::Json;
use leptos::prelude::*;
use leptos_use::use_preferred_dark;
use raw::Raw;

use crate::nu::render::RenderedData;

mod html;
mod json;
mod raw;

#[component]
pub fn Output(output: ReadSignal<RenderedData>) -> impl IntoView {
    let is_preferred_dark = use_preferred_dark();

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum OutputKind {
        Raw,
        Json,
        Html,
    }

    let table_light = Signal::derive(move || output.read().table.light.to_string());
    let table_dark = Signal::derive(move || output.read().table.dark.to_string());
    let json = Signal::derive(move || output.read().json.to_string());
    let html = Signal::derive(move || output.read().html.to_string());

    let (kind, set_kind) = signal(OutputKind::Raw);

    let show_table = Signal::derive(move || kind.get() == OutputKind::Raw);
    let show_table_light = Signal::derive(move || show_table.get() && !is_preferred_dark.get());
    let show_table_dark = Signal::derive(move || show_table.get() && is_preferred_dark.get());
    let show_json = Signal::derive(move || kind.get() == OutputKind::Json);
    let show_html = Signal::derive(move || kind.get() == OutputKind::Html);

    let display = |signal: Signal<bool>| move || if signal.get() { "" } else { "none" };
    let table_light_style = display(show_table_light);
    let table_dark_style = display(show_table_dark);
    let json_style = display(show_json);
    let html_style = display(show_html);

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
            <Raw style:display=table_light_style output=table_light />
            <Raw style:display=table_dark_style output=table_dark />
            <Json style:display=json_style output=json />
            <Html style:display=html_style output=html />
        </div>
    }
}
