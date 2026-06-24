use leptos::either::Either;
use leptos::prelude::*;

mod table;
use table::{SelectedSlots, Table};

mod file_input;
use file_input::{DemoFileInput, PlayerInfo};

mod output_field;
use output_field::OutputField;

#[component]
fn App() -> impl IntoView {
    let (player_info, set_player_info) = signal::<Option<PlayerInfo>>(None);
    let (selected_players, set_selected_players) = signal::<SelectedSlots>(Default::default());

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Demo Parser"</h1>

            <DemoFileInput on_player_info=move |info| set_player_info.set(Some(info)) />
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
                        <OutputField selected_player_slots=selected_players />
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
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
