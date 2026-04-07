use std::collections::HashMap;

use crate::{helpers::list_helper, slay_the_spire::{game_state::GameState, models::{ironclad_relic_pool, shared_relic_pool}, relics::{Relic, RelicRarity}, rng::Rng}};

#[derive(Default, Clone)]
pub struct RelicGrabBag {
    _deques: HashMap<RelicRarity, Vec<Relic>>
}

impl RelicGrabBag {
    pub fn permissible_rarities() -> Vec<RelicRarity> {
        vec![
            RelicRarity::Common,
            RelicRarity::Uncommon,
            RelicRarity::Rare,
            RelicRarity::Shop
        ]
    }

    pub fn populate(&mut self, game_state: &mut GameState) {
        let rng = &mut game_state.rng.up_front;

        let mut list = shared_relic_pool::get_unlocked_relics(&game_state.unlock_state);
        list.extend(ironclad_relic_pool::get_unlocked_relics(&game_state.unlock_state));
        list.retain(|r| Self::permissible_rarities().iter().any(|p| *p == r.rarity()));

        for relic in list {
            #[allow(clippy::unwrap_or_default)]
            self._deques.entry(relic.rarity()).or_insert_with(Vec::new);

            self._deques.entry(relic.rarity()).and_modify(|v| v.push(relic));
        }

        for rarity_list in self._deques.values_mut() {
            list_helper::unstable_shuffle(rarity_list.as_mut_slice(), rng);
        }
    }
}