use crate::slay_the_spire::{events::event::Event, game_state::GameState, rng::Rng};
use crate::slay_the_spire::events::event::EventOption;

#[derive(Debug)]
pub struct JungleMazeAdventure {
    rng: Rng,

    solo_gold: i32,
    join_forces_gold: i32
}

impl Event<'_> for JungleMazeAdventure {
    fn get_internal_name() -> String { "JUNGLE_MAZE_ADVENTURE".to_string() }
    fn is_shared() -> bool { true }

    fn map_event_id() -> Option<&'static str> {
        Some("JungleMazeAdventure")
    }

    fn new(game_state: &GameState) -> Self {
        Self {
            rng: Self::get_rng(game_state.numeric_seed, game_state.player.network_id),
            solo_gold: 150,
            join_forces_gold: 50
        }
    }

    fn calculate_vars(&mut self) {
        self.solo_gold = (self.solo_gold as f32 + self.rng.next_float(-15.0, 15.0)).trunc() as i32;
        self.join_forces_gold = (self.join_forces_gold as f32 + self.rng.next_float(-15.0, 15.0)).trunc() as i32;
    }

    fn generate_initial_options(&mut self) -> Vec<EventOption> {
        vec![
            EventOption::JungleMazeSoloMissionGainGold(self.solo_gold),
            EventOption::JungleMazeTeamUpMissionGainGold(self.join_forces_gold)
        ]
    }
}