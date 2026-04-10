use crate::slay_the_spire::events::event::Event;
use crate::slay_the_spire::events::event::EventOption;
use crate::slay_the_spire::game_state::GameState;
use crate::slay_the_spire::rng::Rng;
use crate::slay_the_spire::relics::Relic;
use crate::helpers::list_helper;

pub struct Neow<'a> {
    game_state: &'a GameState,
    rng: Rng
}

impl Neow<'_> {
    fn positive_options() -> Vec<EventOption> {
        vec![
            EventOption::RelicOption(Relic::ArcaneScroll), EventOption::RelicOption(Relic::BoomingConch),
            EventOption::RelicOption(Relic::Pomander), EventOption::RelicOption(Relic::GoldenPearl),
            EventOption::RelicOption(Relic::LeadPaperweight), EventOption::RelicOption(Relic::NewLeaf),
            EventOption::RelicOption(Relic::NeowsTorment), EventOption::RelicOption(Relic::PreciseScissors),
            EventOption::RelicOption(Relic::LostCoffer)
        ]
    }

    fn toughness_option() -> EventOption { EventOption::RelicOption(Relic::NutritiousOyster) }
    fn safety_option() -> EventOption { EventOption::RelicOption(Relic::StoneHumidifier) }
    fn cleric_option() -> EventOption { EventOption::RelicOption(Relic::MassiveScroll) }
    fn patience_option() -> EventOption { EventOption::RelicOption(Relic::LavaRock) }
    fn scavenger_option() -> EventOption { EventOption::RelicOption(Relic::SmallCapsule) }
    fn empower_option() -> EventOption { EventOption::RelicOption(Relic::SilverCrucible) }

    fn curse_options() -> Vec<EventOption> {
        vec![
            EventOption::RelicOption(Relic::CursedPearl),
            EventOption::RelicOption(Relic::LargeCapsule),
            EventOption::RelicOption(Relic::LeafyPoultice),
            EventOption::RelicOption(Relic::PrecariousShears)
        ]
    }

    fn bundle_option() -> EventOption { EventOption::RelicOption(Relic::ScrollBoxes) }
}

impl<'a> Event<'a> for Neow<'a>{
    fn get_internal_name() -> String { "NEOW".to_string() }

    fn new(game_state: &'a GameState) -> Self {
        Self {
            rng: Self::get_rng(game_state.numeric_seed, game_state.player.network_id),
            game_state
        }
    }

    fn generate_initial_options(&mut self) -> Vec<EventOption> {
        let mut list = Self::curse_options();

        if self.game_state.unlock_state.can_generate_bundles() {
            list.push(Self::bundle_option());
        }

        if self.game_state.player_count == 1 {
            list.push(Self::empower_option());
        }

        let event_option = self.rng.next_item(&list);

        let mut list2 = Self::positive_options();

        let event_option_relic = match event_option {
            EventOption::RelicOption(event_option_relic) => event_option_relic,
            _ => unreachable!()
        };
        
        if event_option_relic == Relic::CursedPearl {
            list2.retain(|&x| x != EventOption::RelicOption(Relic::GoldenPearl))
        }

        if event_option_relic == Relic::PrecariousShears {
            list2.retain(|&x| x != EventOption::RelicOption(Relic::PreciseScissors))
        }

        if event_option_relic == Relic::LeafyPoultice {
            list2.retain(|&x| x != EventOption::RelicOption(Relic::NewLeaf))
        }

        if self.game_state.player_count > 1 {
            list2.push(Self::cleric_option())
        }

        if self.rng.next_bool() {
            list2.push(Self::toughness_option())
        } else {
            list2.push(Self::safety_option())
        }

        if event_option_relic != Relic::LargeCapsule {
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