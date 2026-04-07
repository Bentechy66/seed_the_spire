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
use crate::slay_the_spire::game_state::ParsedSaveData;
use crate::slay_the_spire::relics::Relic;
use crate::slay_the_spire::events::event::EventOption;

fn main() {
    let mut gs = GameState::from_save_file("C:\\Users\\sendb\\AppData\\Roaming\\SlayTheSpire2\\steam\\76561198250957188\\profile1\\saves\\progress.save".to_string(), string_helper::get_deterministic_hash_code("XY9E8QBX6G"));

    gs.initialize_new_run();

    let mut neow = events::Neow::new(&gs);
    neow.calculate_vars();
    let opts = neow.generate_initial_options();

    dbg!(gs.player_relic_grab_bag);
}

#[allow(dead_code)]
fn main_() {
    let seed_cracker = cracker::SeedCracker::with_game_state(GameState::from_save_file("C:\\Users\\sendb\\AppData\\Roaming\\SlayTheSpire2\\steam\\76561198250957188\\profile1\\saves\\progress.save".to_string(), 0))
        // filter by raw hash properties
        .add_condition(|hash| hash % 2 == 0)

        // require that Neow will offer a specific relic
        .add_event(|gs| {
            let mut neow = events::Neow::new(gs);
            neow.calculate_vars();
            neow.generate_initial_options()
        })
        .with_any_option(|opt| matches!(opt, EventOption::RelicOption(Relic::ScrollBoxes)))

        // require that JungleMazeAdventure would generate with more than 150 gold in solo missions
        .add_event(|gs| {
            let mut jma = events::JungleMazeAdventure::new(gs);
            jma.calculate_vars();
            jma.generate_initial_options()
        })
        .with_any_option(|opt| matches!(opt, EventOption::JungleMazeSoloMissionGainGold(g) if *g > 150));

        // let's go!
    let result = seed_cracker.crack();

    let total_attempts = seed_cracker.attempts.load(Ordering::Relaxed);

    println!("Found seed: {result} in {total_attempts} attempts.");
}
