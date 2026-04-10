//! Mirrors `EventModel.IsAllowed(RunState)` from the canonical game for map/room event ids
//! (PascalCase strings used in [`super::room_set`] / [`super::acts::Act::all_events`]).
//!
//! [`could_possibly_be_first_map_event_in_run`] answers whether an event could pass `IsAllowed`
//! while `CurrentActIndex == 0` (still on the first act / Overgrowth timeline), for some
//! single-player run state (optimistic resources and typical starter-deck assumptions).

/// Fields that affect `IsAllowed` in the canonical `RunState` / `Player` snapshot.
#[derive(Clone, Debug)]
pub struct MapEventRunProbe {
    pub current_act_index: u32,
    pub total_floor: u32,
    pub gold: i32,
    pub potion_count: u32,
    pub current_hp: i32,
    pub max_hp: i32,
    pub deck_strike_like_count: u32,
    pub deck_defend_like_count: u32,
    pub has_event_pet: bool,
    pub tradable_relic_count: u32,
    pub has_foul_potion: bool,
    pub deck_has_perfect_fit_target: bool,
    pub deck_has_spiral_enchant_target: bool,
    pub deck_has_basic_removable: bool,
    pub deck_has_removable: bool,
    pub relic_grab_bag_has_available: bool,
    /// Lower bound for Luminous Choir gold after `CalculateVars` (canonical: 149 - NextInt(0,50) → 99..149).
    pub luminous_choir_gold_requirement: i32,
    pub deck_non_empty: bool,
    pub player_count: u32,
}

impl Default for MapEventRunProbe {
    fn default() -> Self {
        Self {
            current_act_index: 0,
            total_floor: 1,
            gold: 99,
            potion_count: 0,
            current_hp: 80,
            max_hp: 80,
            deck_strike_like_count: 5,
            deck_defend_like_count: 5,
            has_event_pet: false,
            tradable_relic_count: 0,
            has_foul_potion: false,
            deck_has_perfect_fit_target: true,
            deck_has_spiral_enchant_target: true,
            deck_has_basic_removable: true,
            deck_has_removable: true,
            relic_grab_bag_has_available: true,
            luminous_choir_gold_requirement: 99,
            deck_non_empty: true,
            player_count: 1,
        }
    }
}

impl MapEventRunProbe {
    /// Best-case single-player probe while still on act index 0 (still “first act” timeline).
    pub fn optimistic_first_act() -> Self {
        Self {
            current_act_index: 0,
            total_floor: 99,
            gold: 101,
            potion_count: 0,
            current_hp: 99,
            max_hp: 100,
            deck_strike_like_count: 10,
            deck_defend_like_count: 10,
            has_event_pet: false,
            tradable_relic_count: 0,
            has_foul_potion: false,
            deck_has_perfect_fit_target: false,
            deck_has_spiral_enchant_target: true,
            deck_has_basic_removable: true,
            deck_has_removable: true,
            relic_grab_bag_has_available: true,
            luminous_choir_gold_requirement: 99,
            deck_non_empty: true,
            player_count: 1,
        }
    }

    fn hp_ratio(&self) -> f64 {
        if self.max_hp <= 0 {
            return 1.0;
        }
        self.current_hp as f64 / self.max_hp as f64
    }
}

/// Canonical `IsAllowed` for a map event id. Events without an override in C# return `true`.
pub fn map_event_is_allowed(event_id: &str, p: &MapEventRunProbe) -> bool {
    match event_id {
        "WarHistorianRepy" => false,

        "DollRoom" => p.current_act_index == 1,

        "WelcomeToWongos" => p.current_act_index == 1 && p.gold >= 100,

        "StoneOfAllTime" => p.current_act_index == 1 && p.potion_count >= 1,

        "BrainLeech" | "RoomFullOfCheese" => p.current_act_index < 2,

        "TeaMaster" => {
            p.current_act_index < 2 && p.gold >= 150
        }

        "TheFutureOfPotions" => p.potion_count >= 2,

        "MorphicGrove" => p.gold >= 100,

        "UnrestSite" => p.hp_ratio() <= 0.70 + f64::EPSILON,

        "CrystalSphere" => p.current_act_index > 0 && p.gold >= 100,

        "FakeMerchant" => {
            if p.current_act_index < 1 || p.player_count > 1 {
                return false;
            }
            p.gold >= 100 || p.has_foul_potion
        }

        "PotionCourier" | "Symbiote" => p.current_act_index > 0,

        "RanwidTheElder" => {
            if p.current_act_index == 0 {
                return false;
            }
            p.tradable_relic_count >= 1 && p.gold >= 100 && p.potion_count >= 1
        }

        "RelicTrader" => {
            if p.current_act_index == 0 {
                return false;
            }
            p.tradable_relic_count >= 5
        }

        "EndlessConveyor" => p.gold >= 105,

        "ColossalFlower" => p.current_hp >= 19,

        "Amalgamator" => p.deck_strike_like_count >= 2 && p.deck_defend_like_count >= 2,

        "ByrdonisNest" => !p.has_event_pet,

        "FieldOfManSizedHoles" => p.deck_has_perfect_fit_target,

        "LuminousChoir" => {
            p.gold >= p.luminous_choir_gold_requirement && p.relic_grab_bag_has_available
        }

        "PunchOff" => p.total_floor >= 6,

        "SlipperyBridge" => {
            p.total_floor > 6 && p.deck_has_removable
        }

        "SpiralingWhirlpool" => p.deck_has_spiral_enchant_target,

        "TheLegendsWereTrue" => {
            p.current_act_index == 0 && p.deck_non_empty && p.current_hp >= 10
        }

        "TrashHeap" => p.current_hp > 5,

        "WaterloggedScriptorium" => p.gold >= 65,

        "WhisperingHollow" => p.gold >= 50,

        "WoodCarvings" => p.deck_has_basic_removable,

        "ZenWeaver" => p.gold >= 125,

        _ => true,
    }
}

/// `true` if this event could pass [`map_event_is_allowed`] while `current_act_index == 0`
/// for some plausible single-player state (see [`MapEventRunProbe::optimistic_first_act`]).
pub fn could_possibly_be_first_map_event_in_run(event_id: &str) -> bool {
    let p = MapEventRunProbe::optimistic_first_act();
    debug_assert_eq!(p.current_act_index, 0);
    map_event_is_allowed(event_id, &p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hard_blocked_on_first_act() {
        let p = MapEventRunProbe::optimistic_first_act();
        for id in [
            "WarHistorianRepy",
            "DollRoom",
            "WelcomeToWongos",
            "StoneOfAllTime",
            "CrystalSphere",
            "FakeMerchant",
            "PotionCourier",
            "Symbiote",
            "RanwidTheElder",
            "RelicTrader",
        ] {
            assert!(
                !map_event_is_allowed(id, &p),
                "{id} should not be allowed on first act with any optimistic probe"
            );
        }
    }

    #[test]
    fn allowed_examples_on_first_act() {
        let p = MapEventRunProbe::optimistic_first_act();
        for id in [
            "BrainLeech",
            "TeaMaster",
            "TheFutureOfPotions",
            "MorphicGrove",
            "PunchOff",
            "RoomFullOfCheese",
            "ByrdonisNest",
        ] {
            assert!(map_event_is_allowed(id, &p), "{id} should be allowed");
        }
    }

    #[test]
    fn unrest_requires_damage() {
        let mut p = MapEventRunProbe::optimistic_first_act();
        p.current_hp = p.max_hp;
        assert!(!map_event_is_allowed("UnrestSite", &p));
        p.current_hp = (p.max_hp as f64 * 0.5) as i32;
        assert!(map_event_is_allowed("UnrestSite", &p));
    }

    #[test]
    fn could_possibly_matches_optimistic() {
        assert!(!could_possibly_be_first_map_event_in_run("WarHistorianRepy"));
        assert!(could_possibly_be_first_map_event_in_run("SapphireSeed"));
    }
}
