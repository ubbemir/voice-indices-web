use demo::demo::extract_player_info;
use js_sys;
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};

#[component]
fn App() -> impl IntoView {
    let (player_info, set_player_info) = signal::<Option<String>>(None);

    let on_file_change = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(file_list) = input.files() {
            if let Some(file) = file_list.get(0) {
                // e.g., file.name(), file.size()
                let promise = file.bytes(); // or file.array_buffer()
                spawn_local(async move {
                    let bytes_value = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                    let byte_array = js_sys::Uint8Array::new(&bytes_value);
                    let mut bytes = vec![0; byte_array.length() as usize];
                    byte_array.copy_to(&mut bytes);

                    // Parse the demo file
                    match extract_player_info(&bytes) {
                        Ok(players) => {
                            let info = players
                                .iter()
                                .map(|p| {
                                    format!(
                                        "{} (Slot: {}, Team: {}, SteamID: {})",
                                        p.name, p.slot, p.team_number, p.steamid
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join("\n");
                            set_player_info.set(Some(info));
                        }
                        Err(e) => {
                            set_player_info.set(Some(format!("Error parsing demo: {}", e)));
                        }
                    }
                });
            }
        }
    };

    view! {
        <div>
            <h1>"Demo Parser"</h1>
            <input type="file" on:change=on_file_change/>
            {move || {
                if let Some(info) = player_info.get() {
                    view! {
                        <pre>{info}</pre>
                    }
                } else {
                    view! {
                        <pre>{"No demo file loaded".to_string()}</pre>
                    }
                }
            }}
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
