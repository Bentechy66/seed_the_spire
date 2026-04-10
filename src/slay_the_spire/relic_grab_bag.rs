use indexmap::IndexMap;

use crate::{helpers::list_helper, slay_the_spire::{characters::Character, game_state::UnlockState, models::{ironclad_relic_pool, shared_relic_pool, silent_relic_pool}, player::Player, relics::{Relic, RelicRarity}, rng::Rng}};

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

    pub fn pull_from_front(&mut self, plr: &mut Player, rarity: &RelicRarity) -> Relic {
        let available_deque = self._deques.get_mut(rarity).unwrap();
        let result = available_deque[0];
        available_deque.remove(0);
        result
    }

    pub fn roll_rarity(&mut self, plr: &mut Player) -> RelicRarity {
        let num = plr.player_rng.rewards.next_float(0.0, 1.0);

        if num < 0.5 {
            RelicRarity::Common
        } else {
            if num > 0.83 {
                RelicRarity::Rare
            } else {
                RelicRarity::Uncommon
            }
        }
    }
}