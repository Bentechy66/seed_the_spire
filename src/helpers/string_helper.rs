pub fn get_deterministic_hash_code(from_str: &str) -> i32 {
    let mut num: i32  = 352654597;
    let mut num2: i32 = num;

    let mut i = 0;
    while i < from_str.len() {
        num = ((num << 5).wrapping_add(num)) ^ (from_str.chars().nth(i).unwrap() as i32);

        if i == from_str.len() - 1 { break; }

        num2 = ((num2 << 5).wrapping_add(num2)) ^ (from_str.chars().nth(i + 1).unwrap() as i32);

        i += 2;
    }

    num.wrapping_add(num2.wrapping_mul(1566083941))
}