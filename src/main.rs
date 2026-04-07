pub mod helpers;
pub mod dotnet;
pub mod slay_the_spire;

use core::num;

use slay_the_spire::events;
use slay_the_spire::events::event::Event;
use slay_the_spire::game_state::GameState;

fn main() {
    // TUGPT9R05U expects [BoomingConch, GoldenPeal, SilverCrucible]
    // 7S78NB2BCP expects [NutritiousOyster, LostCoffer, ScrollBoxes]
    // S3N2SQAK44 expects [ArcaneScroll, SmallCapsule, CursedPearl]
    // JKC19NBHC0 expects [LavaRock, Pomander, PrecariousShears]

    let numeric_seed = helpers::string_helper::get_deterministic_hash_code("TUGPT9R05U".to_string());
    let mut game_state = GameState::default();
    game_state.numeric_seed = numeric_seed;

    println!("Generating Neow items for TUGPT9R05U");
    let mut neow = events::neow::Neow::new(&game_state);
    neow.calculate_vars();
    let options = neow.generate_initial_options();
    println!("{:?}", options);

    println!("Generating jungle maze values for TUGPT9R05U");
    let mut jma = events::jungle_maze_adventure::JungleMazeAdventure::new(&game_state);
    jma.calculate_vars();
    let options = jma.generate_initial_options();

    println!("{:?}", options);
}
