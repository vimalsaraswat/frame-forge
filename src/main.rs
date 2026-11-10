use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Frame Forge"</h1>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
