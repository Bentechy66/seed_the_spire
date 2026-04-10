use crate::{
    helpers::string_helper,
    slay_the_spire::{game_state::UnlockState, rng::Rng},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Act {
    Overgrowth,
    Hive,
    Glory,
    Underdocks,
}

impl Act {
    pub fn number_of_weak_encounters(self) -> i32 {
        match self {
            Act::Glory | Act::Hive => 2,
            Act::Overgrowth | Act::Underdocks => 3,
        }
    }

    pub fn base_number_of_rooms(self) -> i32 {
        match self {
            Act::Glory => 13,
            Act::Hive => 14,
            Act::Overgrowth | Act::Underdocks => 15,
        }
    }

    pub fn number_of_rooms(self, is_multiplayer: bool) -> i32 {
        let mut n = self.base_number_of_rooms();
        if is_multiplayer {
            n -= 1;
        }
        n
    }

    pub fn all_events(self) -> &'static [&'static str] {
        match self {
            Act::Overgrowth => &[
                "AromaOfChaos",
                "ByrdonisNest",
                "DenseVegetation",
                "JungleMazeAdventure",
                "LuminousChoir",
                "MorphicGrove",
                "SapphireSeed",
                "SunkenStatue",
                "TabletOfTruth",
                "UnrestSite",
                "Wellspring",
                "WhisperingHollow",
                "WoodCarvings",
            ],
            Act::Hive => &[
                "Amalgamator",
                "Bugslayer",
                "ColorfulPhilosophers",
                "ColossalFlower",
                "FieldOfManSizedHoles",
                "InfestedAutomaton",
                "LostWisp",
                "SpiritGrafter",
                "TheLanternKey",
                "ZenWeaver",
            ],
            Act::Glory => &[
                "BattlewornDummy",
                "GraveOfTheForgotten",
                "HungryForMushrooms",
                "Reflections",
                "RoundTeaParty",
                "Trial",
                "TinkerTime",
            ],
            Act::Underdocks => &[
                "AbyssalBaths",
                "DrowningBeacon",
                "EndlessConveyor",
                "PunchOff",
                "SpiralingWhirlpool",
                "SunkenStatue",
                "SunkenTreasury",
                "DoorsOfLightAndDark",
                "TrashHeap",
                "WaterloggedScriptorium",
            ],
        }
    }

    pub fn all_encounters(self) -> &'static [&'static str] {
        match self {
            Act::Glory => &[
                "AxebotsNormal",
                "ConstructMenagerieNormal",
                "DevotedSculptorWeak",
                "DoormakerBoss",
                "FabricatorNormal",
                "FrogKnightNormal",
                "GlobeHeadNormal",
                "KnightsElite",
                "MechaKnightElite",
                "OwlMagistrateNormal",
                "QueenBoss",
                "ScrollsOfBitingNormal",
                "ScrollsOfBitingWeak",
                "SlimedBerserkerNormal",
                "SoulNexusElite",
                "TestSubjectBoss",
                "TheLostAndForgottenNormal",
                "TurretOperatorWeak",
            ],
            Act::Hive => &[
                "BowlbugsNormal",
                "BowlbugsWeak",
                "ChompersNormal",
                "DecimillipedeElite",
                "EntomancerElite",
                "ExoskeletonsNormal",
                "ExoskeletonsWeak",
                "HunterKillerNormal",
                "KaiserCrabBoss",
                "InfestedPrismsElite",
                "KnowledgeDemonBoss",
                "LouseProgenitorNormal",
                "MytesNormal",
                "OvicopterNormal",
                "SlumberingBeetleNormal",
                "SpinyToadNormal",
                "TheInsatiableBoss",
                "TheObscuraNormal",
                "ThievingHopperWeak",
                "TunnelerNormal",
                "TunnelerWeak",
            ],
            Act::Overgrowth => &[
                "BygoneEffigyElite",
                "ByrdonisElite",
                "CeremonialBeastBoss",
                "CubexConstructNormal",
                "FlyconidNormal",
                "FogmogNormal",
                "FuzzyWurmCrawlerWeak",
                "InkletsNormal",
                "MawlerNormal",
                "NibbitsNormal",
                "NibbitsWeak",
                "OvergrowthCrawlers",
                "PhrogParasiteElite",
                "RubyRaidersNormal",
                "ShrinkerBeetleWeak",
                "SlimesNormal",
                "SlimesWeak",
                "SlitheringStranglerNormal",
                "SnappingJaxfruitNormal",
                "TheKinBoss",
                "VantomBoss",
                "VineShamblerNormal",
            ],
            Act::Underdocks => &[
                "CorpseSlugsNormal",
                "CorpseSlugsWeak",
                "CultistsNormal",
                "LivingFogNormal",
                "FossilStalkerNormal",
                "GremlinMercNormal",
                "HauntedShipNormal",
                "LagavulinMatriarchBoss",
                "SkulkingColonyElite",
                "PhantasmalGardenersElite",
                "PunchConstructNormal",
                "SeapunkWeak",
                "SewerClamNormal",
                "SludgeSpinnerWeak",
                "SoulFyshBoss",
                "TerrorEelElite",
                "ToadpolesNormal",
                "ToadpolesWeak",
                "TwoTailedRatsNormal",
                "WaterfallGiantBoss",
            ],
        }
    }

    pub fn three_act_order_for_string_seed(
        string_seed: &str,
        unlock: &UnlockState,
        is_multiplayer: bool,
        underdocks_discovered_on_account: bool,
    ) -> [Act; 3] {
        let mut acts = [Act::Overgrowth, Act::Hive, Act::Glory];
        if !unlock.is_epoch_revealed("UNDERDOCKS_EPOCH") {
            return acts;
        }
        let replace_first = if !is_multiplayer && !underdocks_discovered_on_account {
            true
        } else {
            let mut r = Rng::with_seed(string_helper::get_deterministic_hash_code(string_seed) as u32);
            r.next_bool()
        };
        if replace_first {
            acts[0] = Act::Underdocks;
        }
        acts
    }

    pub fn three_act_order_for_numeric_seed(
        numeric_seed: u32,
        unlock: &UnlockState,
        is_multiplayer: bool,
        underdocks_discovered_on_account: bool,
    ) -> [Act; 3] {
        let mut acts = [Act::Overgrowth, Act::Hive, Act::Glory];
        if !unlock.is_epoch_revealed("UNDERDOCKS_EPOCH") {
            return acts;
        }
        let replace_first = if !is_multiplayer && !underdocks_discovered_on_account {
            true
        } else {
            let mut r = Rng::with_seed(numeric_seed);
            r.next_bool()
        };
        if replace_first {
            acts[0] = Act::Underdocks;
        }
        acts
    }
}
