use std::fs;

use serde::{Deserialize, Serialize};

use crate::slay_the_spire::{models::shared_relic_pool, relic_grab_bag::RelicGrabBag, rng::Rng};

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

    pub fn is_epoch_revealed(&self, epoch_name: &str) -> bool {
        self
            .save_data
            .epochs
            .iter()
            .filter(|x| x.id == epoch_name)
            .take(1)
            .any(|x| matches!(x.state, EpochUnlockState::revealed))
    }
}

#[derive(Clone, Debug)]
pub struct RunRngSet {
    pub up_front: Rng
}

impl RunRngSet {
    pub fn from_numeric_seed(seed: u32) -> Self {
        Self {
            up_front: Rng::with_seed_and_name(seed, "up_front".to_string())
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub numeric_seed: i32,
    pub player_network_id: u32, // todo: move this into a more sensible players array
    pub player_count: i32,
    pub rng: RunRngSet,

    pub unlock_state: UnlockState,
    
    pub shared_relic_grab_bag: RelicGrabBag,
    pub player_relic_grab_bag: RelicGrabBag
}

impl GameState {
    pub fn from_save_file(path: String, numeric_seed: i32) -> Self {
        let save_data_raw: String = fs::read_to_string(path).unwrap();

        let d: ParsedSaveData = serde_json::from_str(&save_data_raw).unwrap();

        Self {
            player_count: 1,
            numeric_seed,
            player_network_id: 1,
            unlock_state: UnlockState {
                save_data: d
            },
            rng: RunRngSet::from_numeric_seed(numeric_seed as u32),
            shared_relic_grab_bag: RelicGrabBag::default(),
            player_relic_grab_bag: RelicGrabBag::default()
        }
    }

    pub fn initialize_new_run(&mut self) {
        self
            .shared_relic_grab_bag
            .populate_with_items(shared_relic_pool::get_unlocked_relics(&self.unlock_state), &mut self.rng.up_front);

        self
            .player_relic_grab_bag
            .populate(&self.unlock_state, &mut self.rng.up_front);
    }
}