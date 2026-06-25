use demo::demo::PlayerData;
use js_sys::Uint8Array;
use leptos::either::Either;
use leptos::prelude::*;

use thaw::*;

mod worker;

pub type PlayerInfo = Vec<PlayerData>;

enum Status {
    Nothing,
    Parsing,
    DoneParsing(Result<PlayerInfo, ()>),
}

#[component]
pub fn DemoFileInput(mut on_player_info: impl FnMut(PlayerInfo) + 'static) -> impl IntoView {
    let (parse_process, set_parse_process) = signal::<Option<_>>(None);

    let custom_request = move |file_list: FileList| {
        if let Some(file) = file_list.get(0) {
            set_parse_process.set(Some(LocalResource::new(move || {
                let bytes = file.bytes();
                async move {
                    let byte_array = Uint8Array::new(&bytes.await.unwrap());

                    worker::demo_parser(worker::Request { data: byte_array }).await
                }
            })));
        }
    };

    let status = move || {
        let Some(process) = parse_process.get() else {
            return Status::Nothing;
        };

        // we get a result if process.get returns 'Some' otherwise it is not ready yet
        let Some(result) = process.get() else {
            return Status::Parsing;
        };

        let Ok(worker_response) = result else {
            return Status::DoneParsing(Err(()));
        };

        let Ok(player_info) = worker_response.result else {
            return Status::DoneParsing(Err(()));
        };

        Status::DoneParsing(Ok(player_info))
    };

    Effect::new(move |_| {
        if let Status::DoneParsing(Ok(player_info)) = status() {
            on_player_info(player_info);
        }
    });

    let message = move || match status() {
        Status::Parsing => Some(Either::Left(view! {
            <Spinner size=SpinnerSize::Small />
        })),
        Status::DoneParsing(Err(_)) => Some(Either::Right(view! {
            <MessageBar intent=MessageBarIntent::Error>
                <MessageBarBody>
                    "Error parsing demo"
                </MessageBarBody>
            </MessageBar>
        })),
        _ => None,
    };

    view! {
        <Space>
            <Upload custom_request accept=".dem">
                <Button
                    disabled=move || matches!(status(), Status::Parsing)
                    appearance=ButtonAppearance::Primary
                    icon=icondata::BiFileImportSolid
                >
                    "Select Demo"
                </Button>
            </Upload>
            {message}
        </Space>
    }
}
