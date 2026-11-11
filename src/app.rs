use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <header>
                <h1>"Frame Forge"</h1>
            </header>
        </main>
    }
}

pub fn main() {
    leptos::mount::mount_to_body(App)
}
