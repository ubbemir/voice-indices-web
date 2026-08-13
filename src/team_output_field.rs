use crate::utils::Team;
use demo::demo::PlayerData;
use demo::utils::get_bitfield_from_indices;
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn TeamOutputField(players: ReadSignal<Vec<PlayerData>>, team: Team) -> impl IntoView {
    let team_player_slots = move || {
        players
            .get()
            .iter()
            .filter(|player| team.has_player(player))
            .map(move |player| player.slot)
            .collect::<Vec<_>>()
    };

    let output = move || {
        let bitfield = get_bitfield_from_indices(team_player_slots().iter().copied());
        format!("tv_listen_voice_indices {bitfield}")
    };

    let label_text = team.get_name().unwrap_or("Unknown");
    let badge_color = team.get_badge_color();

    view! {
        <Space justify=SpaceJustify::Center>
            <Badge color=badge_color size=BadgeSize::ExtraSmall />
            <Label size=LabelSize::Small weight=LabelWeight::Semibold>{label_text}</Label>
        </Space>
        <Text tag=TextTag::Code>{output}</Text>
    }
}
