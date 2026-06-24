use demo::demo::PlayerData;
use js_sys::Uint8Array;
use leptos::prelude::*;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};

mod worker;

pub type PlayerInfo = Vec<PlayerData>;

#[component]
pub fn DemoFileInput(mut on_player_info: impl FnMut(PlayerInfo) + 'static) -> impl IntoView {
    let (demo_parse_process, set_demo_parse_process) = signal::<Option<_>>(None);
    let parse_message = move || {
        demo_parse_process
            .get()
            .is_some()
            .then_some("Parsing demo ...")
    };

    let on_file_change = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(file_list) = input.files()
            && let Some(file) = file_list.get(0)
        {
            set_demo_parse_process.set(Some(LocalResource::new(move || {
                let bytes = file.bytes();
                async move {
                    let byte_array = Uint8Array::new(&bytes.await.unwrap());

                    worker::demo_parser(worker::Request { data: byte_array }).await
                }
            })));
        }
    };

    Effect::new(move |_| {
        let Some(process) = demo_parse_process.get() else {
            return;
        };

        // we get a result if process.get returns 'Some' otherwise it is not ready yet
        let Some(result) = process.get() else {
            return;
        };

        let Ok(worker_response) = result else {
            return;
        };

        let Ok(player_info) = worker_response.result else {
            return;
        };

        on_player_info(player_info);
        set_demo_parse_process.set(None);
    });

    view! {
        <input type="file" on:change=on_file_change/>
        <strong>{parse_message}</strong>
    }
}
