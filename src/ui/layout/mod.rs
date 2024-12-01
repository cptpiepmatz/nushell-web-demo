use leptos::prelude::*;

pub fn Navbar() -> impl IntoView {
    let title = "nushell/demo";
    view! {
        <nav class="navbar is-primary">
            <div class="navbar-brand">
                <div class="navbar-item">
                    <p class="title">{title}</p>
                </div>
            </div>
        </nav>
    }
}
