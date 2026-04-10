use crate::slay_the_spire::player_rng_set::PlayerRngSet;

#[derive(Debug, Clone)]
pub struct Player {
    pub network_id: u32,
    pub player_rng: PlayerRngSet
}

impl Player {
    pub fn new(network_id: u32, seed: u32) -> Self {
        Self {
            network_id,
            player_rng: PlayerRngSet::new(seed + network_id)
        }
    }
}