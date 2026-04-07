#[derive(Debug)]
pub struct DotNetRandom {
    inext: usize,
    inextp: usize,
    seed_array: [i32; 56],
}

impl DotNetRandom {
    pub fn new(seed: i32) -> Self {
        let mut seed_array = [0i32; 56];

        let subtraction = if seed == i32::MIN {
            i32::MAX
        } else {
            seed.abs()
        };

        let mut mj = 161803398_i32.wrapping_sub(subtraction);
        seed_array[55] = mj;

        let mut mk = 1_i32;
        for i in 1..55_usize {
            let ii = (21 * i) % 55;
            seed_array[ii] = mk;
            mk = mj.wrapping_sub(mk);
            if mk < 0 {
                mk = mk.wrapping_add(i32::MAX);
            }
            mj = seed_array[ii];
        }

        for _ in 0..4 {
            for i in 1..56_usize {
                let j = (i + 30) % 55 + 1;
                seed_array[i] = seed_array[i].wrapping_sub(seed_array[j]);
                if seed_array[i] < 0 {
                    seed_array[i] = seed_array[i].wrapping_add(i32::MAX);
                }
            }
        }

        Self {
            inext: 0,
            inextp: 21,
            seed_array,
        }
    }

    fn next_sample(&mut self) -> f64 {
        let mut inext = self.inext + 1;
        let mut inextp = self.inextp + 1;

        if inext >= 56 { inext = 1; }
        if inextp >= 56 { inextp = 1; }

        let mut ret = self.seed_array[inext].wrapping_sub(self.seed_array[inextp]);

        if ret == i32::MAX { ret -= 1; }
        if ret < 0 { ret = ret.wrapping_add(i32::MAX); }

        self.seed_array[inext] = ret;
        self.inext = inext;
        self.inextp = inextp;

        ret as f64 * (1.0 / i32::MAX as f64)
    }

    /// Equivalent to Random.Next()
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        (self.next_sample() * i32::MAX as f64) as i32
    }

    /// Equivalent to Random.Next(max_value)
    pub fn next_max(&mut self, max_value: i32) -> i32 {
        assert!(max_value >= 0);
        (self.next_sample() * max_value as f64) as i32
    }

    /// Equivalent to Random.Next(min_value, max_value)
    pub fn next_range(&mut self, min_value: i32, max_value: i32) -> i32 {
        assert!(min_value <= max_value);
        let range = (max_value - min_value) as i64;
        if range <= i32::MAX as i64 {
            (self.next_sample() * range as f64) as i32 + min_value
        } else {
            // Large range path (matches C# source)
            (self.next_sample() * range as f64 + min_value as f64) as i32
        }
    }

    /// Equivalent to Random.NextDouble()
    pub fn next_double(&mut self) -> f64 {
        self.next_sample()
    }
}