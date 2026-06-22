use ahash::AHashMap;
use csgoproto::CsvcMsgVoiceData;
use parser::{
    first_pass::{parser_settings::ParserInputs, read_bits::DemoParserError},
    parse_demo::Parser,
    second_pass::parser_settings::{PlayerEndMetaData, create_huffman_lookup_table},
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct PlayerData {
    pub index: usize,
    pub slot: usize,
    pub steamid: u64,
    pub name: String,
    pub team_number: i32,
}

impl std::fmt::Display for PlayerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} - {}", self.name, self.slot)
    }
}

impl PlayerData {
    fn try_from(md: PlayerEndMetaData, index: usize, slot_offset: usize) -> Result<Self, Error> {
        Ok(PlayerData {
            index,
            slot: slot_from_index(index, slot_offset),
            steamid: md.steamid.ok_or(Error::MissingPlayerField("steamid"))?,
            name: md.name.ok_or(Error::MissingPlayerField("name"))?,
            team_number: md
                .team_number
                .ok_or(Error::MissingPlayerField("team_number"))?,
        })
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("parsing error")]
    DemoParserError(#[from] DemoParserError),

    #[error("unable to compute a slot offset for players")]
    UnattainableSlotOffset,

    #[error("player data in demo is missing a crucial field: {0}")]
    MissingPlayerField(&'static str),
}

pub fn extract_player_info(demo_data: &[u8]) -> Result<Vec<PlayerData>, Error> {
    let settings = ParserInputs {
        real_name_to_og_name: AHashMap::default(),
        wanted_players: vec![],
        wanted_player_props: vec![],
        wanted_other_props: vec![],
        wanted_prop_states: AHashMap::default(),
        wanted_events: vec![],
        parse_ents: false,
        wanted_ticks: vec![],
        parse_projectiles: false,
        parse_grenades: false,
        only_header: true,
        list_props: false,
        only_convars: false,
        huffman_lookup_table: &create_huffman_lookup_table(),
        order_by_steamid: false,
        fallback_bytes: None,
    };

    let mut parser = Parser::new(settings, parser::parse_demo::ParsingMode::Normal);
    let output = parser.parse_demo(demo_data)?;

    let voice_sample = &output.voice_data[0].1;
    let slot_offset = get_player_slot_offset(output.player_md.iter(), voice_sample)
        .ok_or(Error::UnattainableSlotOffset)?;

    let players = output
        .player_md
        .iter()
        .enumerate()
        .map(|(index, p)| PlayerData::try_from(p.clone(), index, slot_offset))
        .collect::<Result<_, Error>>()?;

    Ok(players)
}

fn slot_from_index(index: usize, slot_offset: usize) -> usize {
    const ONE_BASED_INDEXING_OFFSET: usize = 1;

    index + slot_offset + ONE_BASED_INDEXING_OFFSET
}

fn get_player_slot_offset<'a>(
    players: impl Iterator<Item = &'a PlayerEndMetaData>,
    voice_sample: &CsvcMsgVoiceData,
) -> Option<usize> {
    let (index, _) = players
        .enumerate()
        .find(|(_, p)| p.steamid == voice_sample.xuid)?;

    let client_idx = voice_sample.client?;

    (client_idx - index as i32).try_into().ok()
}
