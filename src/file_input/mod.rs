use demo::demo::PlayerData;
use leptos::prelude::*;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};

mod worker;

pub type PlayerInfo = Vec<PlayerData>;

#[component]
pub fn DemoFileInput(mut on_player_info: impl FnMut(PlayerInfo) + 'static) -> impl IntoView {
    let (demo_parse_process, set_demo_parse_process) = signal::<Option<_>>(None);

    let on_file_change = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(file_list) = input.files()
            && let Some(file) = file_list.get(0)
        {
            set_demo_parse_process.set(Some(LocalResource::new(move || {
                let promise = file.bytes();
                async move {
                    let bytes_value = promise.await.unwrap();
                    let byte_array = js_sys::Uint8Array::new(&bytes_value);

                    worker::demo_parser(worker::Request { data: byte_array }).await
                }
            })));
        }
    };

    Effect::new(move |_| {
        let Some(local_resource) = demo_parse_process.get() else {
            return;
        };

        let Some(blabla) = local_resource.get() else {
            return;
        };

        let Ok(response) = blabla else {
            return;
        };

        let Ok(player_info) = response.result else {
            return;
        };

        on_player_info(player_info);
    });

    view! {
        <input type="file" on:change=on_file_change/>
    }
}
