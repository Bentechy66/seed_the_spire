use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum EpochUnlockState {
    revealed,
    not_obtained
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochSaveData {
    id: String,
    obtain_date: u32,
    state: EpochUnlockState
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParsedSaveData {
    epochs: Vec<EpochSaveData>
}

#[derive(Debug, Clone)]
pub struct UnlockState {
    save_data: ParsedSaveData
}

impl UnlockState {
    pub fn can_generate_bundles(&self) -> bool {
        true // TODO
    }
}

#[derive(Clone)]
pub struct GameState {
    pub numeric_seed: i32,
    pub player_network_id: u32, // todo: move this into a more sensible players array
    pub player_count: i32,

    pub unlock_state: UnlockState
}

impl GameState {
    pub fn from_save_file(path: String) -> Self {
        let save_data_raw: String = fs::read_to_string(path).unwrap();

        let d: ParsedSaveData = serde_json::from_str(&save_data_raw).unwrap();

        Self {
            player_count: 1,
            numeric_seed: 42,
            player_network_id: 1,
            unlock_state: UnlockState {
                save_data: d
            }
        }
    }
}