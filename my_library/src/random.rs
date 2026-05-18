use rand::{
    prelude::StdRng,
    Rng,
    SeedableRng,
    distributions::uniform::{SampleRange, SampleUniform}};
use std::ops::Range;

#[cfg(all(not(feature = "pcg"), not(feature = "xorshift")))]
type RngCore = rand::prelude::StdRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

pub struct RandomNumberGenerator {
    rng: RngCore,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: RngCore::from_entropy(),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: RngCore::seed_from_u64(seed)
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

    pub fn val_in_range<T>(&mut self, range: impl SampleRange<T>) -> T
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

    #[test]
    fn test_float() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.val_in_range(-5000.0f32..5000.0f32);
            assert!(n.is_finite());
            assert!(n > -5000.0);
            assert!(n < 5000.0);
        }
    }
}