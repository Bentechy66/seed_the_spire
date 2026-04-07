use crate::dotnet::random::DotNetRandom;
use crate::helpers::string_helper;

#[allow(unused)]
pub struct Rng {
    _random: DotNetRandom,

    counter: i32,
    seed: u32,
}

impl Rng {
    pub fn with_seed(seed: u32) -> Self {
        let mut ret = Rng {
            _random: DotNetRandom::new(seed as i32),
            seed,
            counter: 0,
        };

        ret.fast_forward_counter(ret.counter);

        ret
    }

    pub fn with_seed_and_name(seed: u32, name: String) -> Self {
        Self::with_seed(seed + (string_helper::get_deterministic_hash_code(name) as u32))
    }

    pub fn for_model(seed: u32, net_id: u32, model_name: String) -> Self {
        let mut hash_code: u64 = seed as u64;
        hash_code += net_id as u64; // player net id in singleplayer
        hash_code += string_helper::get_deterministic_hash_code(model_name) as i64 as u64;

        Self::with_seed(hash_code as u32)
    }

    pub fn fast_forward_counter(&mut self, target: i32) {
        while self.counter < target {
            self._random.next();
        }
    }

    pub fn next_int(&mut self, min: i32, max: i32) -> i32 {
        self.counter += 1;

        self._random.next_range(min, max)
    }

    pub fn next_item<T: Copy>(&mut self, from_list: &[T]) -> T {
        let length = from_list.len();
        
        from_list[self.next_int(0, length as i32) as usize]
    }

    pub fn next_bool(&mut self) -> bool {
        self.counter += 1;

        self._random.next_max(2) == 0
    }
}