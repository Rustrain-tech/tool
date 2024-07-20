use itertools::Itertools;
use proconio::{input, source::once::OnceSource};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::io::{BufRead, Write};
use std::path::PathBuf;

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

pub fn parse_input(path: &PathBuf) -> Input {
    let f = std::fs::read_to_string(path).unwrap();
    let f = OnceSource::from(f.as_str());
    input! {
        from f,
        d: usize,
        c: [i64; 26],
        s: [[i64; 26]; d],
    }
    Input { d, c, s }
}

pub fn compute_score(input: &Input, output: &Output) -> i64 {
    let mut last = vec![0; 26];
    let mut score = 0;
    for (i, &t) in output.iter().enumerate().take(input.d) {
        last[t] = i + 1;
        score += input.s[i][t];
        for (j, &l) in last.iter().enumerate().take(26) {
            score -= input.c[j] * (i + 1 - l) as i64;
        }
    }
    score
}

pub fn exec(p: &mut std::process::Child, input_path: &PathBuf) -> Result<i64, String> {
    let input = parse_input(input_path);
    let mut stdin = std::io::BufWriter::new(p.stdin.take().unwrap());
    let mut stdout = std::io::BufReader::new(p.stdout.take().unwrap());
    write!(stdin, "{}", input).unwrap();
    stdin.flush().unwrap();

    p.wait().unwrap();

    let mut output = Vec::new();
    for _ in 0..input.d {
        let mut line = String::new();
        stdout.read_line(&mut line).unwrap();
        let line = line.trim();
        let t = line.parse::<usize>().unwrap();
        output.push(t);
    }
    let score = compute_score(&input, &output);
    Ok(score)
}
