use demo::demo::PlayerData;
use js_sys::Uint8Array;
use leptos::prelude::*;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};

mod worker;

pub type PlayerInfo = Vec<PlayerData>;

enum Status {
    Nothing,
    Parsing,
    DoneParsing(Result<PlayerInfo, ()>),
}

#[component]
pub fn DemoFileInput(mut on_player_info: impl FnMut(PlayerInfo) + 'static) -> impl IntoView {
    let (parse_process, set_parse_process) = signal::<Option<_>>(None);

    let on_file_change = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(file_list) = input.files()
            && let Some(file) = file_list.get(0)
        {
            set_parse_process.set(Some(LocalResource::new(move || {
                let bytes = file.bytes();
                async move {
                    let byte_array = Uint8Array::new(&bytes.await.unwrap());

                    worker::demo_parser(worker::Request { data: byte_array }).await
                }
            })));
        }
    };

    let status = move || {
        let Some(process) = parse_process.get() else {
            return Status::Nothing;
        };

        // we get a result if process.get returns 'Some' otherwise it is not ready yet
        let Some(result) = process.get() else {
            return Status::Parsing;
        };

        let Ok(worker_response) = result else {
            return Status::DoneParsing(Err(()));
        };

        let Ok(player_info) = worker_response.result else {
            return Status::DoneParsing(Err(()));
        };

        Status::DoneParsing(Ok(player_info))
    };

    Effect::new(move |_| {
        if let Status::DoneParsing(Ok(player_info)) = status() {
            on_player_info(player_info);
        }
    });

    let message = move || match status() {
        Status::Parsing => Some("Parsing demo file ..."),
        Status::DoneParsing(Err(_)) => Some("Error parsing demo file!"),
        _ => None,
    };

    view! {
        <input
            type="file"
            accept=".dem"
            on:change=on_file_change
            prop:disabled = move || matches!(status(), Status::Parsing)
        />
        <strong>{message}</strong>
    }
}
