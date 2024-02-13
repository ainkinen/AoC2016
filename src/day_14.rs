use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::aoc;
use md5::{Digest, Md5};

fn hash(s: &str, stretch: i32) -> String {
    let mut hasher = Md5::new();
    hasher.update(s);
    let mut result = hex::encode(hasher.finalize());

    for _ in 0..stretch {
        let mut hasher = Md5::new();
        hasher.update(&result);
        result = hex::encode(hasher.finalize());
    }

    result
}

fn triple(s: &str) -> Option<char> {
    let char_vec = s.chars().collect::<Vec<char>>();

    for win in char_vec.windows(3) {
        if win[0] == win[1] && win[0] == win[2] {
            return Some(win[0]);
        }
    }

    None
}

fn quintuple(s: &str) -> HashSet<char> {
    let mut chars = HashSet::new();
    let char_vec = s.chars().collect::<Vec<char>>();

    for win in char_vec.windows(5) {
        let first_c = win[0];
        if win[1..].iter().all(|&c| c == first_c) {
            chars.insert(first_c);
        }
    }

    chars
}

#[derive(Debug)]
struct PadGen {
    salt: String,
    hash_idx: i32,
    hashes: VecDeque<(String, HashSet<char>)>,
    stretch: i32,
}

impl PadGen {
    fn new(salt: &str, stretch: i32) -> PadGen {
        let first_hashes: VecDeque<(String, HashSet<char>)> = (0..=1000)
            .map(|nonce| hash(&format!("{salt}{nonce}"), stretch))
            .map(|h| {
                let fives = quintuple(&h);
                (h, fives)
            })
            .collect();

        PadGen {
            salt: salt.to_owned(),
            hash_idx: 0,
            hashes: first_hashes,
            stretch,
        }
    }

    fn gen_hash(&mut self) {
        self.hashes.pop_front();
        self.hash_idx += 1;
        let new_hash = hash(
            &format!("{}{}", self.salt, self.hash_idx + 1000),
            self.stretch,
        );
        let fives = quintuple(&new_hash);
        self.hashes.push_back((new_hash, fives));
    }

    fn next_1000(self: &Self) -> impl Iterator<Item = &(String, HashSet<char>)> {
        self.hashes.iter().skip(1)
    }
}

fn solve(salt: &str, stretch: i32) -> i32 {
    let mut pg = PadGen::new(salt, stretch);

    let mut pad: Vec<(i32, String)> = Vec::new();

    loop {
        // let char3 = repeats(&pg.hashes[0].0, 3);
        let char3 = triple(&pg.hashes[0].0);

        if char3
            .iter()
            .any(|c| pg.next_1000().any(|(_, fives)| fives.contains(c)))
        {
            pad.push((pg.hash_idx, pg.hashes[0].0.clone()))
        }

        if pad.len() == 64 {
            return pg.hash_idx;
        }

        pg.gen_hash();
    }
}

#[aoc(day14, part1)]
fn part1(salt: &str) -> i32 {
    solve(salt, 0)
}

#[aoc(day14, part2)]
fn part2(salt: &str) -> i32 {
    solve(salt, 2016)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "abc";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 22728);
    }

    #[ignore]
    #[test]
    fn test_part2() {
        // slow
        assert_eq!(part2(TEST_INPUT), 22551);
    }
}
