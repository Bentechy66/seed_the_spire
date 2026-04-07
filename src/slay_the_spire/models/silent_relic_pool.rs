use crate::slay_the_spire::{game_state::UnlockState, relics::Relic};

fn generate_all_relics() -> Vec<Relic> {
    vec![
        Relic::HelicalDart,
        Relic::NinjaScroll,
        Relic::PaperKrane,
        Relic::RingOfTheSnake,
        Relic::SneckoSkull,
        Relic::Tingsha,
        Relic::ToughBandages,
        Relic::TwistedFunnel,
    ]
}

pub fn get_unlocked_relics(unlock_state: &UnlockState) -> Vec<Relic> {
    let mut list = generate_all_relics();

    if !unlock_state.is_epoch_revealed("SILENT3_EPOCH") {
        list.retain(|x| !(matches!(x, Relic::ToughBandages) || matches!(x, Relic::PaperKrane) || matches!(x, Relic::Tingsha)))
    }

    if !unlock_state.is_epoch_revealed("SILENT6_EPOCH") {
        list.retain(|x| !(matches!(x, Relic::TwistedFunnel) || matches!(x, Relic::SneckoSkull) || matches!(x, Relic::HelicalDart)))
    }

    list
}