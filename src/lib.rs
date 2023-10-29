#![feature(generic_arg_infer)]
#![feature(new_uninit)]
mod error;
mod mhf;
mod utils;

pub use error::Error;
pub use error::Result;

use std::path::PathBuf;

use serde::Deserialize;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum MhfFlags {
    Selfup = 1,
    Restat = 2,
    Autolc = 3,
    Hanres = 4,
    DmmBoot = 5,
    DmmSelfup = 6,
    DmmAutolc = 7,
    DmmReboot = 8,
    Npge = 9,
    NpMhfoTest = 10,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum MezFesStalls {
    TokotokoPartnya = 2,
    Pachinko = 3,
    VolpakkunTogether = 4,
    Nyanrendo = 6,
    HoneyPanic = 7,
    DokkanBattleCats = 8,
    PointStall = 9,
    StallMap = 10,
}

#[derive(Debug, Deserialize)]
pub struct MhfConfigMessage {
    pub flags: u16,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct MhfConfig {
    pub char_id: u32,
    pub char_name: String,
    pub char_gr: u32,
    pub char_hr: u32,
    pub char_ids: Vec<u32>,
    pub char_new: bool,
    pub user_token: String,
    pub user_name: String,
    pub user_password: String,
    pub user_rights: u32,
    pub entrance_count: u32,
    pub current_ts: u32,
    pub expiry_ts: u32,
    pub messages: Vec<MhfConfigMessage>,
    pub mez_event_id: u32,
    pub mez_start: u32,
    pub mez_end: u32,
    pub mez_solo_tickets: u32,
    pub mez_group_tickets: u32,
    pub mez_stalls: Vec<MezFesStalls>,

    // Optional
    pub mhf_folder: Option<PathBuf>,
    pub mhf_flags: Option<Vec<MhfFlags>>,
}

pub fn run(config: MhfConfig) -> Result<isize> {
    if config.user_token.len() != 16 {
        return Err(Error::TokenLength);
    }
    mhf::run_mhf(config)
}
