use itertools::Itertools;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub struct Input {
    d: usize,
    c: Vec<i64>,
    s: Vec<Vec<i64>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.d)?;
        writeln!(f, "{}", self.c.iter().join(" "))?;
        for i in 0..self.d {
            writeln!(f, "{}", self.s[i].iter().join(" "))?;
        }
        Ok(())
    }
}

pub type Output = Vec<usize>;

pub fn gen(seed: u64) -> Input {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let d = 365;
    let c = (0..26).map(|_| rng.gen_range(0..=100)).collect();
    let s = (0..d)
        .map(|_| (0..26).map(|_| rng.gen_range(0..=1000)).collect())
        .collect();
    Input { d, c, s }
}
