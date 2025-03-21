use pumpkin_data::packet::serverbound::PLAY_CLIENT_INFORMATION;
use pumpkin_macros::packet;
use serde::Serialize;

use crate::VarInt;

#[derive(serde::Deserialize, Serialize)]
#[packet(PLAY_CLIENT_INFORMATION)]
pub struct SClientInformationPlay {
    pub locale: String, // 16
    pub view_distance: i8,
    pub chat_mode: VarInt, // VarInt
    pub chat_colors: bool,
    pub skin_parts: u8,
    pub main_hand: VarInt,
    pub text_filtering: bool,
    pub server_listing: bool,
}
