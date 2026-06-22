use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    let on_file_change = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(file_list) = input.files() {
            if let Some(file) = file_list.get(0) {
                // e.g., file.name(), file.size()
                let promise = file.bytes(); // or file.array_buffer()
                spawn_local(async move {
                    let text = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                    // Process the file content here
                });
            }
        }
    };

    view! {
        <input type="file" on:change=on_file_change/>
    }
}

fn main() {
    mount_to_body(App);
}
