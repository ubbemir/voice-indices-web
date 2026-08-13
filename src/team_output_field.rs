use demo::demo::PlayerData;
use demo::utils::get_bitfield_from_indices;
use leptos::prelude::*;
use thaw::*;

pub enum Team {
    Even,
    Odd,
}

impl Team {
    fn has_player(&self, player: &PlayerData) -> bool {
        match self {
            Self::Even => player.team_number % 2 == 0,
            Self::Odd => player.team_number % 2 == 1,
        }
    }
}

#[component]
pub fn TeamOutputField(
    players: ReadSignal<Vec<PlayerData>>,
    team: Team,
    label: &'static str,
) -> impl IntoView {
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

    view! {
        <Label size=LabelSize::Small weight=LabelWeight::Semibold>{label}</Label>
        <Text tag=TextTag::Code>{output}</Text>
    }
}
