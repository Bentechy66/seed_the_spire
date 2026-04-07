pub mod helpers;
pub mod dotnet;
pub mod slay_the_spire;

fn main() {
    // TUGPT9R05U expects [BoomingConch, GoldenPeal, SilverCrucible]
    // 7S78NB2BCP expects [NutritiousOyster, LostCoffer, ScrollBoxes]
    // S3N2SQAK44 expects [ArcaneScroll, SmallCapsule, CursedPearl]
    // JKC19NBHC0 expects [LavaRock, Pomander, PrecariousShears]

    println!("Generating Neow items for TUGPT9R05U");
    let numeric_seed = helpers::string_helper::get_deterministic_hash_code("TUGPT9R05U".to_string());
    let event_rng = slay_the_spire::rng::Rng::for_model(numeric_seed as u32, 1, "NEOW".to_string());
    let mut neow = slay_the_spire::events::neow::Neow::new(event_rng);
    let options = neow.generate_initial_options(true, 1);
    println!("{:?}", options);

    println!("Generating jungle maze values for TUGPT9R05U");
    let numeric_seed = helpers::string_helper::get_deterministic_hash_code("TUGPT9R05U".to_string());
    let event_rng = slay_the_spire::rng::Rng::for_model(numeric_seed as u32, 0, "JUNGLE_MAZE_ADVENTURE".to_string());
    let mut jma = slay_the_spire::events::jungle_maze_adventure::JungleMazeAdventure::new(event_rng);
    jma.calculate_vars();

    println!("{:?}", jma);
}
