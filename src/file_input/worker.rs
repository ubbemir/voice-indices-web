use super::PlayerInfo;
use demo::demo::extract_player_info;
use leptos_workers::worker;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Request {
    #[serde(with = "leptos_workers::transferable")]
    pub data: js_sys::Uint8Array,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub result: Result<PlayerInfo, ()>,
}

#[worker]
pub async fn demo_parser(req: Request) -> Response {
    let mut bytes = Vec::with_capacity(req.data.length() as usize);
    req.data.copy_to(&mut bytes);

    let parse_result = extract_player_info(&bytes);

    Response {
        result: parse_result.map_err(|_| ()),
    }
}
