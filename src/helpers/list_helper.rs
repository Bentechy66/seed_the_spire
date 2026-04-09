use crate::slay_the_spire::rng::Rng;

pub fn unstable_shuffle<T: Copy>(list: &mut [T], rng: &mut Rng) {
    let mut num = list.len();

    while num > 1 {
        num -= 1;

        let num2 = rng.next_int(0, (num + 1) as i32);
        // dbg!(num, num2);
        let index = num2;
        let index2 = num;

        let value = list[num];
        let value2 = list[num2 as usize];

        list[index as usize] = value;
        list[index2] = value2;
    }
}