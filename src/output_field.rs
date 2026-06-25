use crate::table::SelectedSlots;
use demo::utils::get_bitfield_from_indices;
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn OutputField(selected_player_slots: ReadSignal<SelectedSlots>) -> impl IntoView {
    let output = move || {
        let bitfield = get_bitfield_from_indices(selected_player_slots.get().iter().copied());
        format!("tv_listen_voice_indices {bitfield}")
    };

    view! {
        <Label size=LabelSize::Small weight=LabelWeight::Semibold>"Command"</Label>
        <Text tag=TextTag::Code>{output}</Text>
    }
}
