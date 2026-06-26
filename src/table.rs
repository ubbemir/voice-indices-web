use std::collections::HashSet;

use demo::demo::PlayerData;
use leptos::either::Either;
use leptos::prelude::*;
use thaw::*;

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
                    itertools::intersperse_with(teams().into_iter().map(|(_, players)| {
                        let rows = players.into_iter().map(|player| {
                            let is_selected = move || selected_player_slots.get().contains(&player.slot);

                            let badge_color = if player.team_number % 2 == 0 {
                                BadgeColor::Brand
                            } else {
                                BadgeColor::Danger
                            };

                            view! {
                                <tr on:click = move |_| on_player_select(player.slot)>
                                    <td>
                                        <Checkbox checked={is_selected()} />
                                    </td>
                                    <td>
                                        <Space justify=SpaceJustify::Center>
                                            <Label weight=LabelWeight::Semibold>{player.name}</Label>
                                            <Badge color=badge_color size=BadgeSize::ExtraSmall />
                                        </Space>
                                    </td>
                                    <td>{player.slot}</td>
                                    <td>{player.steamid}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>();
                        Either::Left(view! {
                            {rows}
                        })
                    }), || Either::Right(view! {
                        <tr>
                            <td colspan=4>
                                <Divider />
                            </td>
                        </tr>
                    })).collect_view()
                }}
            </tbody>
        </table>
    }
}
