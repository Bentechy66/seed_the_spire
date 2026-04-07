pub mod helpers;
pub mod dotnet;

fn main() {
    // let mut rng = dotnet::random::DotNetRandom::new(42);
    let mut hash_code: u64 = helpers::string_helper::get_deterministic_hash_code("TUGPT9R05U".to_string()) as u64;
    hash_code += helpers::string_helper::get_deterministic_hash_code("NEOW".to_string()) as i64 as u64;

    let mut rng = dotnet::random::DotNetRandom::new(hash_code as u32 as i32);

    println!("{}", rng.next_range(0, 6));
    println!("{}", rng.next_max(2) == 0);
    println!("{}", rng.next_max(2) == 0);
}
