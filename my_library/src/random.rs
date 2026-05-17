use rand::{prelude::StdRng, Rng, SeedableRng};
use std::ops::Range;

pub struct RandomNumberGenerator {
    rng: StdRng,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_entropy(),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed)
        }
    }

    // pub fn u32_in_range(&mut self, range: Range<u32>) -> u32 {
    //     self.rng.gen_range(range)
    // }

    pub fn next<T>(&mut self) -> T
        where rand::distributions::Standard: rand::prelude::Distribution<T>
    {
        self.rng.r#gen()
    }

    pub fn val_in_range<T>(&mut self, range: Range<T>) -> T
        where T: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        self.rng.gen_range(range)
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bounds() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 10..1000 {
            let (min, max) = (1, 10);
            let n = rng.val_in_range(min..max);
            assert!(n >= min);
            assert!(n < max);
        }
    }

    #[test]
    fn test_reproducibility() {
        let mut rng = (
            RandomNumberGenerator::seeded(1),
            RandomNumberGenerator::seeded(1),
        );
        (0..1000).for_each(|_| {
            assert_eq!(
                rng.0.val_in_range(u32::MIN..u32::MAX),
                rng.1.val_in_range(u32::MIN..u32::MAX),
            );
        });
    }

    #[test]
    fn test_next_types() {
        let mut rng = RandomNumberGenerator::new();
        let _ : i32 = rng.next();
        let _ = rng.next::<f32>();
    }
}