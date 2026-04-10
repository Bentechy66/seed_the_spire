use crate::{
    helpers::{list_helper, string_helper},
    slay_the_spire::{
        game_state::{GameState, UnlockState},
        grab_bag::GrabBag,
        models::{
            acts::Act,
            ancients::{Ancient, shared_ancient_pool, unlocked_ancients_for_act},
            encounters::shares_tags,
        },
        rng::Rng,
    },
};

#[derive(Debug, Clone)]
pub struct RoomSet {
    pub events: Vec<&'static str>,
    pub normal_encounters: Vec<&'static str>,
    pub elite_encounters: Vec<&'static str>,
    pub boss: &'static str,
    pub ancient: Ancient,
}

const SHARED_EVENTS: &[&str] = &[
    "BrainLeech",
    "CrystalSphere",
    "DollRoom",
    "FakeMerchant",
    "PotionCourier",
    "RanwidTheElder",
    "RelicTrader",
    "RoomFullOfCheese",
    "SelfHelpBook",
    "SlipperyBridge",
    "StoneOfAllTime",
    "Symbiote",
    "TeaMaster",
    "TheFutureOfPotions",
    "TheLegendsWereTrue",
    "ThisOrThat",
    "WarHistorianRepy",
    "WelcomeToWongos",
];

fn filter_events_for_epochs(events: &mut Vec<&'static str>, unlock: &UnlockState) {
    if !unlock.is_epoch_revealed("EVENT1_EPOCH") {
        events.retain(|&e| e != "TrashHeap");
    }
    if !unlock.is_epoch_revealed("EVENT2_EPOCH") {
        events.retain(|&e| e != "Reflections");
    }
    if !unlock.is_epoch_revealed("EVENT3_EPOCH") {
        events.retain(|&e| e != "ColorfulPhilosophers");
    }
}

fn is_weak_monster(id: &str) -> bool {
    !id.ends_with("Boss") && !id.ends_with("Elite") && id.ends_with("Weak")
}

fn is_elite(id: &str) -> bool {
    id.ends_with("Elite")
}

fn is_boss(id: &str) -> bool {
    id.ends_with("Boss")
}

fn refill_grab_bag<'a>(grab_bag: &mut GrabBag<&'a str>, pool: &[&'a str]) {
    grab_bag.refill_uniform(pool);
}

fn add_without_repeating_tags(
    encounters: &mut Vec<&'static str>,
    grab_bag: &mut GrabBag<&'static str>,
    rng: &mut Rng,
) {
    let last = encounters.last().copied();
    let picked = grab_bag.grab_and_remove_if(rng, |e| {
        !shares_tags(e, last) && Some(e) != last
    });
    let picked = picked.or_else(|| grab_bag.grab_and_remove(rng));
    if let Some(e) = picked {
        encounters.push(e);
    }
}

pub fn generate_rooms(
    act: Act,
    rng: &mut Rng,
    unlock: &UnlockState,
    is_multiplayer: bool,
    shared_ancient_subset: &[Ancient],
) -> RoomSet {
    let mut events: Vec<&'static str> = act.all_events().iter().copied().collect();
    events.extend_from_slice(SHARED_EVENTS);
    filter_events_for_epochs(&mut events, unlock);
    list_helper::unstable_shuffle(events.as_mut_slice(), rng);

    let all = act.all_encounters();
    let weak_pool: Vec<&str> = all.iter().copied().filter(|id| is_weak_monster(id)).collect();
    let regular_pool: Vec<&str> = all
        .iter()
        .copied()
        .filter(|id| !is_weak_monster(id) && !is_elite(id) && !is_boss(id))
        .collect();
    let elite_pool: Vec<&str> = all.iter().copied().filter(|id| is_elite(id)).collect();
    let boss_pool: Vec<&str> = all.iter().copied().filter(|id| is_boss(id)).collect();

    let mut normal_encounters = Vec::new();
    let mut grab_bag = GrabBag::new();

    for _ in 0..act.number_of_weak_encounters() {
        if !grab_bag.any() {
            refill_grab_bag(&mut grab_bag, &weak_pool);
        }
        add_without_repeating_tags(&mut normal_encounters, &mut grab_bag, rng);
    }

    let n_rooms = act.number_of_rooms(is_multiplayer);
    let mut regular_grab_bag = GrabBag::new();
    for _ in act.number_of_weak_encounters()..n_rooms {
        if !regular_grab_bag.any() {
            refill_grab_bag(&mut regular_grab_bag, &regular_pool);
        }
        add_without_repeating_tags(&mut normal_encounters, &mut regular_grab_bag, rng);
    }

    let mut elite_encounters = Vec::new();
    let mut elite_grab_bag = GrabBag::new();
    for _ in 0..15 {
        if !elite_grab_bag.any() {
            refill_grab_bag(&mut elite_grab_bag, &elite_pool);
        }
        add_without_repeating_tags(&mut elite_encounters, &mut elite_grab_bag, rng);
    }

    let boss = rng.next_item(&boss_pool);

    let mut ancient_pool = unlocked_ancients_for_act(act, unlock);
    ancient_pool.extend_from_slice(shared_ancient_subset);
    assert!(
        !ancient_pool.is_empty(),
        "generate_rooms: empty ancient pool (act {:?})",
        act
    );
    let ancient = rng.next_item(&ancient_pool);

    RoomSet {
        events,
        normal_encounters,
        elite_encounters,
        boss,
        ancient,
    }
}

pub fn assign_shared_ancient_subsets(rng: &mut Rng, unlock: &UnlockState, subsequent_acts: usize) -> Vec<Vec<Ancient>> {
    let mut pool = shared_ancient_pool(unlock);
    list_helper::unstable_shuffle(pool.as_mut_slice(), rng);
    let mut out = Vec::with_capacity(subsequent_acts);
    for _ in 0..subsequent_acts {
        let count = rng.next_int(0, (pool.len() + 1) as i32) as usize;
        let take_count = count.min(pool.len());
        let taken: Vec<Ancient> = pool.drain(..take_count).collect();
        out.push(taken);
    }
    out
}

pub const DEFAULT_ACT_ORDER: [Act; 3] = [Act::Overgrowth, Act::Hive, Act::Glory];

pub fn generate_run_room_sets_with_up_front(
    rng: &mut Rng,
    unlock: &UnlockState,
    is_multiplayer: bool,
    acts: &[Act; 3],
) -> Vec<(Act, RoomSet)> {
    let subsets = assign_shared_ancient_subsets(rng, unlock, acts.len() - 1);
    let mut out = Vec::with_capacity(acts.len());
    for (i, act) in acts.iter().enumerate() {
        let subset: &[Ancient] = if i == 0 {
            &[]
        } else {
            subsets[i - 1].as_slice()
        };
        let rooms = generate_rooms(*act, rng, unlock, is_multiplayer, subset);
        out.push((*act, rooms));
    }
    out
}

pub fn generate_run_room_sets_match_game(mut gs: GameState, acts: [Act; 3]) -> Vec<(Act, RoomSet)> {
    gs.initialize_new_run();
    generate_run_room_sets_with_up_front(
        &mut gs.rng.up_front,
        &gs.unlock_state,
        gs.player_count > 1,
        &acts,
    )
}

pub fn generate_run_room_sets_match_game_from_string_seed(
    string_seed: &str,
    player_count: i32,
    unlock: UnlockState,
    underdocks_discovered_on_account: bool,
) -> Vec<(Act, RoomSet)> {
    let acts = Act::three_act_order_for_string_seed(
        string_seed,
        &unlock,
        player_count > 1,
        underdocks_discovered_on_account,
    );
    let gs = GameState::new_run_preview(string_seed, player_count, unlock);
    generate_run_room_sets_match_game(gs, acts)
}

pub fn print_run_room_sets_for_string_seed(
    string_seed: &str,
    player_count: i32,
    unlock: &UnlockState,
    underdocks_discovered_on_account: bool,
) {
    let numeric_seed = string_helper::get_deterministic_hash_code(string_seed) as u32;
    println!("string_seed = {string_seed:?}");
    println!("numeric_seed (u32) = {numeric_seed}");
    let acts = Act::three_act_order_for_string_seed(
        string_seed,
        unlock,
        player_count > 1,
        underdocks_discovered_on_account,
    );
    println!("act order: {:?} -> {:?} -> {:?}", acts[0], acts[1], acts[2]);
    let gs = GameState::new_run_preview(string_seed, player_count, unlock.clone());
    let run = generate_run_room_sets_match_game(gs, acts);
    for (act, rooms) in run {
        println!("\n--- {:?} ---", act);
        println!("events (visit order wraps):");
        for (i, e) in rooms.events.iter().enumerate() {
            println!("  {:3} {}", i, e);
        }
        println!("normals ({}):", rooms.normal_encounters.len());
        for (i, e) in rooms.normal_encounters.iter().enumerate() {
            println!("  {:3} {}", i, e);
        }
        println!("elites:");
        for (i, e) in rooms.elite_encounters.iter().enumerate() {
            println!("  {:3} {}", i, e);
        }
        println!("boss: {}", rooms.boss);
        println!("ancient: {:?}", rooms.ancient);
    }
}
