use crate::slay_the_spire::rng::Rng;

#[derive(Debug, Clone)]
pub struct PlayerRngSet {
    pub rewards: Rng
}

impl PlayerRngSet {
    pub fn new(seed: u32) -> Self {
        Self {
            rewards: Rng::with_seed_and_name(seed, "rewards".to_string())
        }
    }
}