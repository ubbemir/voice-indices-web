use demo::demo::{PlayerData, extract_player_info};
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};

pub type PlayerInfo = Vec<PlayerData>;

#[component]
pub fn DemoFileInput(set_player_info: WriteSignal<Option<PlayerInfo>>) -> impl IntoView {
    let on_file_change = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(file_list) = input.files()
            && let Some(file) = file_list.get(0)
        {
            let promise = file.bytes();
            spawn_local(async move {
                let bytes_value = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                let byte_array = js_sys::Uint8Array::new(&bytes_value);
                let mut bytes = vec![0; byte_array.length() as usize];
                byte_array.copy_to(&mut bytes);

                if let Ok(players) = extract_player_info(&bytes) {
                    set_player_info.set(Some(players));
                }
            });
        }
    };

    view! {
        <input type="file" on:change=on_file_change/>
    }
}
