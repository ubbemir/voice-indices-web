use demo::demo::{PlayerData, extract_player_info};
use js_sys;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};

mod table;
use table::{SelectedSlots, Table};

#[component]
fn App() -> impl IntoView {
    let (player_info, set_player_info) = signal::<Option<Vec<PlayerData>>>(None);
    let (selected_players, set_selected_players) = signal::<SelectedSlots>(Default::default());

    let on_file_change = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(file_list) = input.files() {
            if let Some(file) = file_list.get(0) {
                let promise = file.bytes();
                spawn_local(async move {
                    let bytes_value = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                    let byte_array = js_sys::Uint8Array::new(&bytes_value);
                    let mut bytes = vec![0; byte_array.length() as usize];
                    byte_array.copy_to(&mut bytes);

                    match extract_player_info(&bytes) {
                        Ok(players) => {
                            set_player_info.set(Some(players));
                            set_selected_players.set(Default::default());
                        }
                        Err(_) => (),
                    }
                });
            }
        }
    };

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Demo Parser"</h1>
            <input type="file" on:change=on_file_change/>

            {move || {
                if let Some(players) = player_info.get() {
                    Either::Left(view! {
                        <Table
                            players={
                                let (sig, _) = signal(players.clone());
                                sig
                            }
                            selected_player_slots=selected_players
                            set_selected_player_slots=set_selected_players
                        />
                        <p>{selected_players.get().len()}</p>
                    })
                } else {
                    Either::Right(view! {
                        <p>"No demo file loaded"</p>
                    })
                }
            }}
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
