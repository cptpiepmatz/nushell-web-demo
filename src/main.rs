use wasm_bindgen::JsValue;

mod nu;
mod ui;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(ui::App)
}
