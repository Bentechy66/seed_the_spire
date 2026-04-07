use crate::slay_the_spire::rng::Rng;
use crate::slay_the_spire::relics::Relic;
use crate::helpers::list_helper;

pub struct Neow {
    rng: Rng
}

impl Neow {
    pub fn positive_options() -> Vec<Relic> {
        vec![
            Relic::ArcaneScroll, Relic::BoomingConch,
            Relic::Pomander, Relic::GoldenPearl,
            Relic::LeadPaperweight, Relic::NewLeaf,
            Relic::NeowsTorment, Relic::PreciseScissors,
            Relic::LostCoffer
        ]
    }

    pub fn toughness_option() -> Relic { Relic::NutritiousOyster }
    pub fn safety_option() -> Relic { Relic::StoneHumidifer }
    pub fn cleric_option() -> Relic { Relic::MassiveScroll }
    pub fn patience_option() -> Relic { Relic::LavaRock }
    pub fn scavenger_option() -> Relic { Relic::SmallCapsule }
    pub fn empower_option() -> Relic { Relic::SilverCrucible }

    pub fn curse_options() -> Vec<Relic> {
        vec![
            Relic::CursedPearl,
            Relic::LargeCapsule,
            Relic::LeafyPoultice,
            Relic::PrecariousShears
        ]
    }

    pub fn bundle_option() -> Relic { Relic::ScrollBoxes }

    pub fn new(rng: Rng) -> Self {
        Self {
            rng
        }
    }

    pub fn generate_initial_options(&mut self, can_generate_bundles: bool, player_count: i32) -> Vec<Relic> {
        let mut list = Self::curse_options();

        if can_generate_bundles {
            list.push(Self::bundle_option());
        }

        if player_count == 1 {
            list.push(Self::empower_option());
        }

        let event_option = self.rng.next_item(&list);

        let mut list2 = Self::positive_options();

        if event_option == Relic::CursedPearl {
            list2.retain(|&x| x != Relic::GoldenPearl)
        }

        if event_option == Relic::PrecariousShears {
            list2.retain(|&x| x != Relic::PreciseScissors)
        }

        if event_option == Relic::LeafyPoultice {
            list2.retain(|&x| x != Relic::NewLeaf)
        }

        if player_count > 1 {
            list2.push(Self::cleric_option())
        }

        if self.rng.next_bool() {
            list2.push(Self::toughness_option())
        } else {
            list2.push(Self::safety_option())
        }

        if event_option != Relic::LargeCapsule {
            if self.rng.next_bool() {
                list2.push(Self::patience_option())
            } else {
                list2.push(Self::scavenger_option())
            }
        }

        list_helper::unstable_shuffle(list2.as_mut_slice(), &mut self.rng);

        list2
            .into_iter()
            .take(2)
            .chain(std::iter::once(event_option))
            .collect()
    }
}