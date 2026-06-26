use leptos::either::Either;
use leptos::prelude::*;
use thaw::*;

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

    let on_player_info = move |info| {
        set_selected_players.set(Default::default());
        set_player_info.set(Some(info));
    };

    let theme = RwSignal::new(Theme::dark());

    view! {
        <ConfigProvider theme>
            <Layout class="app">
                <Card>
                    <CardHeader>
                        <Body1>
                            <h1>"CS2 Voice Calculator"</h1>
                        </Body1>

                        <CardHeaderAction slot>
                            <DemoFileInput on_player_info=on_player_info />
                        </CardHeaderAction>
                    </CardHeader>

                    {move || {
                        if let Some(players) = player_info.get() {
                            Either::Left(view! {
                                <Divider />
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
                            Either::Right(())
                        }
                    }}
                </Card>
            </Layout>
        </ConfigProvider>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    if web_sys::window().is_some() {
        mount_to_body(App);
    }
}
