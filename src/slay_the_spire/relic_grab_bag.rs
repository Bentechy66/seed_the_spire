use indexmap::IndexMap;

use crate::{helpers::list_helper, slay_the_spire::{characters::Character, game_state::UnlockState, models::{ironclad_relic_pool, shared_relic_pool, silent_relic_pool}, relics::{Relic, RelicRarity}, rng::Rng}};

#[derive(Default, Clone, Debug)]
pub struct RelicGrabBag {
    _deques: IndexMap<RelicRarity, Vec<Relic>>,
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

    pub fn populate_with_items(&mut self, relics: Vec<Relic>, rng: &mut Rng) {
        for original_relic in relics {
            #[allow(clippy::unwrap_or_default)]
            self._deques.entry(original_relic.rarity()).or_insert_with(Vec::new);

            self._deques.entry(original_relic.rarity()).and_modify(|v| v.push(original_relic));
        }

        for rarity_list in self._deques.values_mut() {
            list_helper::unstable_shuffle(rarity_list.as_mut_slice(), rng);
        }
    }

    pub fn populate(&mut self, unlock_state: &UnlockState, character: Character, rng: &mut Rng) {
        let mut list = shared_relic_pool::get_unlocked_relics(unlock_state);
        match character {
            Character::Ironclad => list.extend(ironclad_relic_pool::get_unlocked_relics(unlock_state)),
            Character::Silent => list.extend(silent_relic_pool::get_unlocked_relics(unlock_state)),
        }
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