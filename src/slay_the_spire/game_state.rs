use std::fs;

use serde::{Deserialize, Serialize};

use crate::{
    helpers::string_helper,
    slay_the_spire::{
        characters::Character,
        models::{
            acts::Act,
            ancients::Ancient,
            room_set::{self, DEFAULT_ACT_ORDER},
            shared_relic_pool,
        },
        player::Player,
        relic_grab_bag::RelicGrabBag,
        rng::Rng,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum EpochUnlockState {
    revealed,
    not_obtained,
    obtained
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
    save_data: ParsedSaveData,
    all_epochs: bool,
}

impl UnlockState {
    pub fn can_generate_bundles(&self) -> bool {
        true // TODO
    }

    pub fn is_epoch_revealed(&self, epoch_name: &str) -> bool {
        self.all_epochs ||
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
    pub player: Player, // todo: move this into a more sensible players array
    pub player_count: i32,
    pub rng: RunRngSet,
    pub active_character: Character,

    pub unlock_state: UnlockState,
    
    pub shared_relic_grab_bag: RelicGrabBag,
    pub player_relic_grab_bag: RelicGrabBag,
    pub event_room_order: Vec<(Act, Vec<&'static str>)>,
    /// Filled by [`GameState::initialize_new_run`]: ancients for each timeline act in order.
    pub run_ancients: Vec<(Act, Ancient)>,
}

impl GameState {
    pub fn from_save_file(path: String, numeric_seed: i32, as_character: Character) -> Self {
        let save_data_raw: String = fs::read_to_string(path).unwrap();

        let d: ParsedSaveData = serde_json::from_str(&save_data_raw).unwrap();

        Self {
            player_count: 1,
            numeric_seed,
            player: Player::new(1, numeric_seed as u32),
            unlock_state: UnlockState {
                save_data: d,
                all_epochs: false,
            },
            rng: RunRngSet::from_numeric_seed(numeric_seed as u32),
            shared_relic_grab_bag: RelicGrabBag::default(),
            player_relic_grab_bag: RelicGrabBag::default(),
            event_room_order: vec![],
            run_ancients: vec![],
            active_character: as_character
        }
    }

    pub fn with_all_unlocks(numeric_seed: i32, as_character: Character) -> Self {
        Self {
            player_count: 1,
            numeric_seed,
            player: Player::new(1, numeric_seed as u32),
            unlock_state: UnlockState {
                save_data: ParsedSaveData{ epochs: vec![] },
                all_epochs: true
            },
            rng: RunRngSet::from_numeric_seed(numeric_seed as u32),
            shared_relic_grab_bag: RelicGrabBag::default(),
            player_relic_grab_bag: RelicGrabBag::default(),
            event_room_order: vec![],
            run_ancients: vec![],
            active_character: as_character
        }
    }

    pub fn new_run_preview(string_seed: &str, player_count: i32, unlock_state: UnlockState) -> Self {
        let numeric_seed = string_helper::get_deterministic_hash_code(string_seed);
        Self {
            player_count,
            numeric_seed,
            player: Player::new(1, numeric_seed as u32),
            unlock_state,
            rng: RunRngSet::from_numeric_seed(numeric_seed as u32),
            shared_relic_grab_bag: RelicGrabBag::default(),
            player_relic_grab_bag: RelicGrabBag::default(),
            event_room_order: vec![],
            run_ancients: vec![],
            active_character: Character::Ironclad,
        }
    }

    /// Resets RNG streams and per-player RNG so they match `numeric_seed` (e.g. after assigning a trial hash).
    pub fn sync_rng_from_numeric_seed(&mut self) {
        self.rng = RunRngSet::from_numeric_seed(self.numeric_seed as u32);
        self.player = Player::new(self.player.network_id, self.numeric_seed as u32);
    }

    pub fn initialize_new_run(&mut self) {
        self
            .shared_relic_grab_bag
            .populate_with_items(shared_relic_pool::get_unlocked_relics(&self.unlock_state), &mut self.rng.up_front);

        self
            .player_relic_grab_bag
            .populate(&self.unlock_state, self.active_character, &mut self.rng.up_front);

        let acts = Act::three_act_order_for_numeric_seed(
            self.numeric_seed as u32,
            &self.unlock_state,
            self.player_count > 1,
            self.unlock_state.is_epoch_revealed("UNDERDOCKS_EPOCH"),
        );
        let room_sets = room_set::generate_run_room_sets_with_up_front(
            &mut self.rng.up_front,
            &self.unlock_state,
            self.player_count > 1,
            &acts,
        );
        self.run_ancients = room_sets
            .iter()
            .map(|(act, rooms)| (*act, rooms.ancient))
            .collect();
        self.event_room_order = room_sets
            .into_iter()
            .map(|(act, rooms)| (act, rooms.events))
            .collect();
    }

    pub fn event_room_order_for_act(&self, act: Act) -> Option<&Vec<&'static str>> {
        self.event_room_order
            .iter()
            .find(|(a, _)| *a == act)
            .map(|(_, events)| events)
    }

    pub fn default_three_act_order() -> [Act; 3] {
        DEFAULT_ACT_ORDER
    }
}