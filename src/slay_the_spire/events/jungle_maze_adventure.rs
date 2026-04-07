use crate::slay_the_spire::rng::Rng;

#[derive(Debug)]
pub struct JungleMazeAdventure {
    rng: Rng,

    solo_gold: i32,
    join_forces_gold: i32
}

impl JungleMazeAdventure {
    pub fn new(rng: Rng) -> Self {
        Self {
            rng,
            solo_gold: 150,
            join_forces_gold: 50
        }
    }

    pub fn calculate_vars(&mut self) {
        self.solo_gold = (self.solo_gold as f32 + self.rng.next_float(-15.0, 15.0)).trunc() as i32;
        self.join_forces_gold = (self.join_forces_gold as f32 + self.rng.next_float(-15.0, 15.0)).trunc() as i32;
    }
}