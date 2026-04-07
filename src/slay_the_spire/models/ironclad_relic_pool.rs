use crate::slay_the_spire::{game_state::UnlockState, relics::Relic};

fn generate_all_relics() -> Vec<Relic> {
    vec![
        Relic::Brimstone,
        Relic::BurningBlood,
        Relic::CharonsAshes,
        Relic::DemonTongue,
        Relic::PaperPhrog,
        Relic::RedSkull,
        Relic::RuinedHelmet,
        Relic::SelfFormingClay,
    ]
}

pub fn get_unlocked_relics(unlock_state: &UnlockState) -> Vec<Relic> {
    let mut list = generate_all_relics();

    if !unlock_state.is_epoch_revealed("IRONCLAD3_EPOCH") {
        list.retain(|x| !(matches!(x, Relic::RedSkull) || matches!(x, Relic::PaperPhrog) || matches!(x, Relic::RuinedHelmet)))
    }

    if !unlock_state.is_epoch_revealed("IRONCLAD6_EPOCH") {
        list.retain(|x| !(matches!(x, Relic::SelfFormingClay) || matches!(x, Relic::CharonsAshes) || matches!(x, Relic::DemonTongue)))
    }

    list
}