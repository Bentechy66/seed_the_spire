use crate::slay_the_spire::{game_state::UnlockState, relics::Relic};

fn generate_all_relics() -> Vec<Relic> {
    vec![
        Relic::Akabeko,
        Relic::AmethystAubergine,
        Relic::Anchor,
        Relic::ArtOfWar,
        Relic::BagOfMarbles,
        Relic::BagOfPreparation,
        Relic::BeatingRemnant,
        Relic::Bellows,
        Relic::BeltBuckle,
        Relic::BloodVial,
        Relic::BookOfFiveRings,
        Relic::BowlerHat,
        Relic::Bread,
        Relic::BronzeScales,
        Relic::BurningSticks,
        Relic::Candelabra,
        Relic::CaptainsWheel,
        Relic::Cauldron,
        Relic::CentennialPuzzle,
        Relic::Chandelier,
        Relic::ChemicalX,
        Relic::CloakClasp,
        Relic::DingyRug,
        Relic::DollysMirror,
        Relic::DragonFruit,
        Relic::EternalFeather,
        Relic::FestivePopper,
        Relic::FresnelLens,
        Relic::FrozenEgg,
        Relic::GamblingChip,
        Relic::GamePiece,
        Relic::GhostSeed,
        Relic::Girya,
        Relic::GnarledHammer,
        Relic::Gorget,
        Relic::GremlinHorn,
        Relic::HappyFlower,
        Relic::HornCleat,
        Relic::IceCream,
        Relic::IntimidatingHelmet,
        Relic::JossPaper,
        Relic::JuzuBracelet,
        Relic::Kifuda,
        Relic::Kunai,
        Relic::Kusarigama,
        Relic::Lantern,
        Relic::LastingCandy,
        Relic::LavaLamp,
        Relic::LeesWaffle,
        Relic::LetterOpener,
        Relic::LizardTail,
        Relic::LoomingFruit,
        Relic::LuckyFysh,
        Relic::Mango,
        Relic::MealTicket,
        Relic::MeatOnTheBone,
        Relic::MembershipCard,
        Relic::MercuryHourglass,
        Relic::MiniatureCannon,
        Relic::MiniatureTent,
        Relic::MoltenEgg,
        Relic::MummifiedHand,
        Relic::MysticLighter,
        Relic::Nunchaku,
        Relic::OddlySmoothStone,
        Relic::OldCoin,
        Relic::Orichalcum,
        Relic::OrnamentalFan,
        Relic::Orrery,
        Relic::Pantograph,
        Relic::ParryingShield,
        Relic::Pear,
        Relic::PenNib,
        Relic::Pendulum,
        Relic::Permafrost,
        Relic::PetrifiedToad,
        Relic::Planisphere,
        Relic::Pocketwatch,
        Relic::PotionBelt,
        Relic::PrayerWheel,
        Relic::PunchDagger,
        Relic::RainbowRing,
        Relic::RazorTooth,
        Relic::RedMask,
        Relic::RegalPillow,
        Relic::ReptileTrinket,
        Relic::RingingTriangle,
        Relic::RippleBasin,
        Relic::RoyalStamp,
        Relic::ScreamingFlagon,
        Relic::Shovel,
        Relic::Shuriken,
        Relic::SlingOfCourage,
        Relic::SparklingRouge,
        Relic::StoneCalendar,
        Relic::StoneCracker,
        Relic::Strawberry,
        Relic::StrikeDummy,
        Relic::SturdyClamp,
        Relic::TheAbacus,
        Relic::TheCourier,
        Relic::TinyMailbox,
        Relic::Toolbox,
        Relic::ToxicEgg,
        Relic::TungstenRod,
        Relic::TuningFork,
        Relic::UnceasingTop,
        Relic::UnsettlingLamp,
        Relic::Vajra,
        Relic::Vambrace,
        Relic::VenerableTeaSet,
        Relic::VeryHotCocoa,
        Relic::VexingPuzzlebox,
        Relic::WarPaint,
        Relic::Whetstone,
        Relic::WhiteBeastStatue,
        Relic::WhiteStar,
        Relic::WingCharm,
    ]
}

pub fn get_unlocked_relics(unlock_state: &UnlockState) -> Vec<Relic> {
    let mut list = generate_all_relics();

    if !unlock_state.is_epoch_revealed("RELIC1_EPOCH") {
        list.retain(|x| !(matches!(x, Relic::UnsettlingLamp) || matches!(x, Relic::IntimidatingHelmet) || matches!(x, Relic::ReptileTrinket)))
    }

    if !unlock_state.is_epoch_revealed("RELIC2_EPOCH") {
        list.retain(|x| !(matches!(x, Relic::BookOfFiveRings) || matches!(x, Relic::IceCream) || matches!(x, Relic::Kusarigama)))
    }

    if !unlock_state.is_epoch_revealed("RELIC3_EPOCH") {
        list.retain(|x| !(matches!(x, Relic::VexingPuzzlebox) || matches!(x, Relic::RippleBasin) || matches!(x, Relic::FestivePopper)))
    }

    if !unlock_state.is_epoch_revealed("RELIC4_EPOCH") {
        list.retain(|x| !(matches!(x, Relic::MiniatureCannon) || matches!(x, Relic::TungstenRod) || matches!(x, Relic::WhiteStar)))
    }

    if !unlock_state.is_epoch_revealed("RELIC5_EPOCH") {
        list.retain(|x| !(matches!(x, Relic::TinyMailbox) || matches!(x, Relic::JossPaper) || matches!(x, Relic::BeatingRemnant)))
    }

    list
}