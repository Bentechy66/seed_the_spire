use crate::slay_the_spire::{game_state::GameState, relics::Relic, rng::Rng};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventOption {
    // Shared options
    /// Option which gives you a relic if you pick it
    RelicOption(Relic),

    // Jungle Maze Adventure Options
    /// The amount of gold given for a solo mission (you will lose 18 hp)
    JungleMazeSoloMissionGainGold(i32),
    /// The amount of gold given if you team up (you will lose 0 hp)
    JungleMazeTeamUpMissionGainGold(i32),
}

pub trait Event<'a> {
    fn get_internal_name() -> String;
    fn is_shared() -> bool { false }

    fn calculate_vars(&mut self) { }

    fn get_rng(numeric_seed: i32, owner_net_id: u32) -> Rng {
        Rng::for_model(
            numeric_seed as u32, 
            if Self::is_shared() { 0 } else { owner_net_id }, 
            Self::get_internal_name()
        )
    }

    fn new(game_state: &'a GameState) -> Self;

    #[allow(unused)]
    fn generate_initial_options(&mut self) -> Vec<EventOption> { vec![] }
}