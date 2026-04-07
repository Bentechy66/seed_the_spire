use crate::slay_the_spire::{game_state::UnlockState, models::acts::Act};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ancient {
    Nonupeipe,
    Tanx,
    Vakuu,
    Orobas,
    Pael,
    Tezcatara,
    Neow,
    Darv,
}

pub fn shared_ancient_pool(unlock: &UnlockState) -> Vec<Ancient> {
    if unlock.is_epoch_revealed("DARV_EPOCH") {
        vec![Ancient::Darv]
    } else {
        vec![]
    }
}

pub fn unlocked_ancients_for_act(act: Act, unlock: &UnlockState) -> Vec<Ancient> {
    match act {
        Act::Glory => vec![Ancient::Nonupeipe, Ancient::Tanx, Ancient::Vakuu],
        Act::Hive => {
            let mut v = vec![Ancient::Orobas, Ancient::Pael, Ancient::Tezcatara];
            if !unlock.is_epoch_revealed("OROBAS_EPOCH") {
                v.retain(|a| *a != Ancient::Orobas);
            }
            v
        }
        Act::Overgrowth | Act::Underdocks => {
            let mut v = vec![Ancient::Neow];
            if !unlock.is_epoch_revealed("NEOW_EPOCH") {
                v.retain(|a| *a != Ancient::Neow);
            }
            v
        }
    }
}
