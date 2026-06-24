use std::collections::HashSet;

use demo::demo::PlayerData;
use leptos::prelude::*;

pub type SelectedSlots = HashSet<usize>;

#[component]
pub fn Table(
    players: ReadSignal<Vec<PlayerData>>,
    selected_player_slots: ReadSignal<SelectedSlots>,
    set_selected_player_slots: WriteSignal<SelectedSlots>,
) -> impl IntoView {
    let on_player_select = move |slot: usize| {
        let mut selected = selected_player_slots.get();
        if selected.contains(&slot) {
            selected.remove(&slot);
        } else {
            selected.insert(slot);
        }
        set_selected_player_slots.set(selected.clone());
    };

    let teams = move || {
        let mut teams = std::collections::HashMap::new();
        for player in players.get().iter() {
            teams
                .entry(player.team_number)
                .or_insert_with(Vec::new)
                .push(player.clone());
        }

        let mut teams = teams
            .into_iter()
            .map(|(team_num, mut players)| {
                players.sort_by_key(|p| p.slot);
                (team_num, players)
            })
            .collect::<Vec<_>>();

        teams.sort_by_key(|(team, _)| *team);

        teams
    };

    view! {
        <table style="border-collapse: collapse; margin-top: 20px; margin-bottom: 20px;">
            <thead>
                <tr>
                    <th>
                    </th>
                    <th>
                        "Name"
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
                    teams().into_iter().map(|(_, players)| {
                        let rows = players.into_iter().map(|player| {
                            let is_selected = move || selected_player_slots.get().contains(&player.slot);
                            let style = if player.team_number % 2 == 0 {
                                "background-color: #bdbdbd"
                            } else {
                                ""
                            };

                            view! {
                                <tr style=style>
                                    <td>
                                        <input
                                            type="checkbox"
                                            checked=is_selected()
                                            on:change=move |_| on_player_select(player.slot)
                                        />
                                    </td>
                                    <td>{player.name}</td>
                                    <td>{player.slot}</td>
                                    <td>{player.steamid}</td>
                                </tr>
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
