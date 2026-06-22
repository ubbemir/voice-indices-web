use demo::demo::{PlayerData, extract_player_info};
use demo::utils::get_voice_filter;
use js_sys;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{HtmlInputElement, wasm_bindgen::JsCast};

#[component]
fn App() -> impl IntoView {
    let (player_info, set_player_info) = signal::<Option<Vec<PlayerData>>>(None);
    let (selected_players, set_selected_players) = signal::<Vec<usize>>(Vec::new());
    let (voice_filter, set_voice_filter) = signal::<i32>(0);

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
                            set_selected_players.set(Vec::new());
                            set_voice_filter.set(0);
                        }
                        Err(_) => (),
                    }
                });
            }
        }
    };

    let on_player_select = move |index: usize| {
        let mut selected = selected_players.get();
        if selected.contains(&index) {
            selected.retain(|&i| i != index);
        } else {
            selected.push(index);
        }
        set_selected_players.set(selected.clone());

        if let Some(players) = player_info.get() {
            let filter_players: Vec<_> = selected.iter().filter_map(|&i| players.get(i)).collect();
            let bitfield = get_voice_filter(filter_players.iter().copied());
            set_voice_filter.set(bitfield);
        }
    };

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Demo Parser"</h1>
            <input type="file" on:change=on_file_change/>

            {move || {
                if let Some(players) = player_info.get() {
                    let mut teams: std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();
                    for (idx, p) in players.iter().enumerate() {
                        teams.entry(p.team_number).or_insert_with(Vec::new).push(idx);
                    }

                    let mut team_ids: Vec<_> = teams.keys().copied().collect();
                    team_ids.sort();

                    Either::Left(view! {
                        <div>
                            <table style="border-collapse: collapse; margin-top: 20px; margin-bottom: 20px;">
                                <thead>
                                    <tr>
                                        <th style="border: 1px solid #ddd; padding: 8px; text-align: left;">
                                            "Select"
                                        </th>
                                        <th style="border: 1px solid #ddd; padding: 8px; text-align: left;">
                                            "Name"
                                        </th>
                                        <th style="border: 1px solid #ddd; padding: 8px; text-align: left;">
                                            "Team"
                                        </th>
                                        <th style="border: 1px solid #ddd; padding: 8px; text-align: left;">
                                            "Slot"
                                        </th>
                                        <th style="border: 1px solid #ddd; padding: 8px; text-align: left;">
                                            "SteamID"
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {team_ids.into_iter().map(|team_id| {
                                        let team_player_indices = teams.get(&team_id).unwrap().clone();
                                        let players_clone = players.clone();
                                        let rows: Vec<_> = team_player_indices.iter().map(|idx| {
                                            if let Some(p) = players_clone.get(*idx) {
                                                let idx_copy = *idx;
                                                let is_selected = move || selected_players.get().contains(&idx_copy);
                                                Either::Left(view! {
                                                    <tr style={if is_selected() { "background-color: #f0f0f0;" } else { "" }}>
                                                        <td style="border: 1px solid #ddd; padding: 8px;">
                                                            <input
                                                                type="checkbox"
                                                                checked=is_selected()
                                                                on:change=move |_| on_player_select(idx_copy)
                                                            />
                                                        </td>
                                                        <td style="border: 1px solid #ddd; padding: 8px;">{p.name.clone()}</td>
                                                        <td style="border: 1px solid #ddd; padding: 8px;">{p.team_number}</td>
                                                        <td style="border: 1px solid #ddd; padding: 8px;">{p.slot}</td>
                                                        <td style="border: 1px solid #ddd; padding: 8px;">{p.steamid}</td>
                                                    </tr>
                                                })
                                            } else {
                                                Either::Right(view! {})
                                            }
                                        }).collect::<Vec<_>>();
                                        view! {
                                            {rows}
                                        }
                                    }).collect_view()}
                                </tbody>
                            </table>

                            <div style="margin-top: 20px;">
                                <label for="filter-output" style="display: block; margin-bottom: 8px;">
                                    "Voice Filter Output:"
                                </label>
                                <textarea
                                    id="filter-output"
                                    disabled=true
                                    style="width: 100%; height: 100px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; font-family: monospace;"
                                >
                                    {voice_filter.get().to_string()}
                                </textarea>
                            </div>
                        </div>
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
