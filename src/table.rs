use std::collections::HashSet;

use demo::demo::PlayerData;
use leptos::either::Either;
use leptos::prelude::*;

pub type SelectedSlots = HashSet<usize>;

#[component]
pub fn Table(
    players: ReadSignal<Vec<PlayerData>>,
    selected_player_slots: ReadSignal<SelectedSlots>,
    set_selected_player_slots: WriteSignal<SelectedSlots>,
) -> impl IntoView {
    let on_player_select = move |index: usize| {
        let mut selected = selected_player_slots.get();
        if selected.contains(&index) {
            selected.remove(&index);
        } else {
            selected.insert(index);
        }
        set_selected_player_slots.set(selected.clone());
    };

    let players_data = move || {
        let players_vec = players.get();
        let mut teams = std::collections::HashMap::new();
        for (idx, p) in players_vec.iter().enumerate() {
            teams
                .entry(p.team_number)
                .or_insert_with(Vec::new)
                .push(idx);
        }

        let mut team_ids: Vec<_> = teams.keys().copied().collect();
        team_ids.sort();
        (teams, team_ids, players_vec)
    };

    view! {
        <table style="border-collapse: collapse; margin-top: 20px; margin-bottom: 20px;">
            <thead>
                <tr>
                    <th>
                        "Select"
                    </th>
                    <th>
                        "Name"
                    </th>
                    <th>
                        "Team"
                    </th>
                    <th>
                        "Slot"
                    </th>
                    <th>
                        "SteamID"
                    </th>
                </tr>
            </thead>
            <tbody>
                {move || {
                    let (teams, team_ids, players_vec) = players_data();
                    team_ids.into_iter().map(|team_id| {
                        let team_player_indices = teams.get(&team_id).unwrap().clone();
                        let rows: Vec<_> = team_player_indices.iter().map(|idx| {
                            if let Some(p) = players_vec.get(*idx) {
                                let idx_copy = *idx;
                                let is_selected = move || selected_player_slots.get().contains(&idx_copy);
                                let player_slot = p.slot.clone();

                                Either::Left(view! {
                                    <tr>
                                        <td>
                                            <input
                                                type="checkbox"
                                                checked=is_selected()
                                                on:change=move |_| on_player_select(player_slot)
                                            />
                                        </td>
                                        <td>{p.name.clone()}</td>
                                        <td>{p.team_number}</td>
                                        <td>{p.slot}</td>
                                        <td>{p.steamid}</td>
                                    </tr>
                                })
                            } else {
                                Either::Right(view! {})
                            }
                        }).collect::<Vec<_>>();
                        view! {
                            {rows}
                        }
                    }).collect_view()
                }}
            </tbody>
        </table>
    }
}
