#![feature(mpsc_is_disconnected)]

pub mod helpers;
pub mod dotnet;
pub mod slay_the_spire;
pub mod cracker;

use std::fs;
use std::sync::atomic::Ordering;

use slay_the_spire::events;
use slay_the_spire::events::event::Event;
use slay_the_spire::game_state::GameState;

use crate::helpers::string_helper;
use crate::slay_the_spire::characters::Character;
use crate::slay_the_spire::game_state::ParsedSaveData;
use crate::slay_the_spire::models::acts::Act;
use crate::slay_the_spire::models::ancients::Ancient;
use crate::slay_the_spire::models::map_event_eligibility::{self, map_event_is_allowed};
use crate::slay_the_spire::relics::Relic;
use crate::slay_the_spire::events::event::EventOption;

fn main_() {
    // let mut gs = GameState::from_save_file("C:\\Users\\sendb\\AppData\\Roaming\\SlayTheSpire2\\steam\\76561198250957188\\profile1\\saves\\progress.save".to_string(), string_helper::get_deterministic_hash_code("E545V00WVS9"), Character::Silent);
    let mut gs = GameState::from_save_file("C:\\Users\\sendb\\AppData\\Roaming\\SlayTheSpire2\\steam\\76561198250957188\\profile1\\saves\\progress.save".to_string(), string_helper::get_deterministic_hash_code("R6MQQNHSRS"), Character::Ironclad);

    gs.initialize_new_run();

    let mut neow = events::Neow::new(&gs);
    neow.calculate_vars();
    let opts = neow.generate_initial_options();

    dbg!(opts);
    let rarity = &gs.player_relic_grab_bag.roll_rarity(&mut gs.player);
    let relic = gs.player_relic_grab_bag.pull_from_front(&mut gs.player, rarity);
    dbg!(rarity, gs.player_relic_grab_bag, relic);
    dbg!(gs.event_room_order[0].1.iter().filter(|e| map_event_is_allowed(e, &map_event_eligibility::MapEventRunProbe::optimistic_first_act())).collect::<Vec<_>>());
}

#[allow(dead_code)]
fn main() {
    let seed_cracker = cracker::SeedCracker::with_game_state(GameState::from_save_file("C:\\Users\\sendb\\AppData\\Roaming\\SlayTheSpire2\\steam\\76561198250957188\\profile1\\saves\\progress.save".to_string(), 0, Character::Ironclad))
        // filter by raw hash properties
        // .add_condition(|hash| hash % 2 == 0)

        // // require that Neow will offer a specific relic
        .add_event(|gs| {
            let mut neow = events::Neow::new(gs);
            neow.calculate_vars();
            neow.generate_initial_options()
        })
        .with_any_option(|opt| matches!(opt, EventOption::RelicOption(Relic::ScrollBoxes)))

        // .require_all_events_in_first_n_for_act(Act::Overgrowth, 1, &["JungleMazeAdventure"])
        .require_run_includes_ancients(&[Ancient::Neow, Ancient::Orobas]);

        // let's go!
    let result = seed_cracker.crack();

    let total_attempts = seed_cracker.attempts.load(Ordering::Relaxed);

    println!("Found seed: {result} in {total_attempts} attempts.");
}
