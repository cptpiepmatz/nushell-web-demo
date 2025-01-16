use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let title = "nushell/demo";
    let repo_url = env!("CARGO_PKG_REPOSITORY");
    view! {
        <nav class="navbar is-primary">
            <div class="navbar-brand">
                <div class="navbar-item">
                    <p class="title">{title}</p>
                </div>
            </div>
            <div class="navbar-end">
                <a 
                    class="navbar-item" 
                    href={repo_url}
                    target="_blank"
                >
                    <span class="icon is-medium">
                        <img
                            class="text-filter-override"
                            src="./octicons/mark-github-24.svg" 
                            style="height: 100%; max-height: none"
                        />
                    </span>
                </a>
            </div>
        </nav>
    }
}
