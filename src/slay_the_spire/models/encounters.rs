#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncounterTag {
    Burrower,
    Chomper,
    Nibbit,
    Shrinker,
    Slimes,
    Thieves,
    Workers,
    Crawler,
    Mushroom,
    Knights,
    Scrolls,
    Seapunk,
    Slugs,
    Exoskeletons,
}

pub fn tags_for_encounter(id: &'static str) -> &'static [EncounterTag] {
    match id {
        "TunnelerWeak" => &[EncounterTag::Burrower],
        "TunnelerNormal" => &[EncounterTag::Burrower, EncounterTag::Workers],
        "ThievingHopperWeak" => &[EncounterTag::Thieves],
        "CorpseSlugsWeak" | "CorpseSlugsNormal" => &[EncounterTag::Slugs],
        "SnappingJaxfruitNormal" => &[EncounterTag::Mushroom],
        "SlumberingBeetleNormal" => &[EncounterTag::Workers],
        "ChompersNormal" => &[EncounterTag::Chomper],
        "OvergrowthCrawlers" => &[EncounterTag::Shrinker, EncounterTag::Crawler],
        "SlimesWeak" | "SlimesNormal" => &[EncounterTag::Slimes],
        "BowlbugsWeak" | "BowlbugsNormal" => &[EncounterTag::Workers],
        "NibbitsWeak" => &[EncounterTag::Nibbit],
        "FlyconidNormal" => &[EncounterTag::Mushroom, EncounterTag::Slimes],
        "ShrinkerBeetleWeak" => &[EncounterTag::Shrinker],
        "SeapunkWeak" => &[EncounterTag::Seapunk],
        "ScrollsOfBitingWeak" | "ScrollsOfBitingNormal" => &[EncounterTag::Scrolls],
        "ExoskeletonsWeak" | "ExoskeletonsNormal" => &[EncounterTag::Exoskeletons],
        "KnightsElite" => &[EncounterTag::Knights],
        "FuzzyWurmCrawlerWeak" => &[EncounterTag::Crawler],
        _ => &[],
    }
}

pub fn shares_tags(a: &'static str, b: Option<&'static str>) -> bool {
    let Some(other) = b else {
        return false;
    };
    let ta = tags_for_encounter(a);
    let tb = tags_for_encounter(other);
    ta.iter().any(|t| tb.contains(t))
}
