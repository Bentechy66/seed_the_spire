pub struct GameState {
    pub numeric_seed: i32,
    pub player_network_id: u32, // todo: move this into a more sensible players array
    pub player_count: i32,
    pub can_generate_bundles: bool
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player_count: 1,
            can_generate_bundles: true,
            numeric_seed: 42,
            player_network_id: 1
        }
    }
}