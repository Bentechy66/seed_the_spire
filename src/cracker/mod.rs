use rand::RngExt;

pub fn generate_random_seed() -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKMNPQRSTUVWXYZ0123456789";
    const LEN: usize = 10;

    let mut rng = rand::rng();
    let mut buf = [0u8; LEN];

    for b in buf.iter_mut() {
        *b = CHARS[rng.random_range(0..CHARS.len())];
    }

    unsafe { String::from_utf8_unchecked(buf.to_vec()) }
}

pub struct SeedCracker {
    conditions: Vec<Box<dyn Fn(i32) -> bool>>
}

impl Default for SeedCracker {
    fn default() -> Self {
        Self {
            conditions: Vec::new()
        }
    }
}

impl SeedCracker {
    pub fn add_condition(mut self, condition: impl Fn(i32) -> bool + 'static) -> Self {
        self.conditions.push(Box::new(condition));
        self
    }

    pub fn crack(&self) -> String {
        loop {
            let seed = generate_random_seed();
            let derived = crate::helpers::string_helper::get_deterministic_hash_code(&seed);
            if self.conditions.iter().all(|c| c(derived)) {
                return seed;
            }
        }
    }
}