use aoc_runner_derive::aoc;
use hex;
use md5::{Digest, Md5};
use std::collections::HashMap;

fn hash(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

fn find_next(id: &str, nonce: i32) -> (String, i32) {
    let mut nonce = nonce;
    loop {
        let mut s = id.to_owned();
        s.push_str(&nonce.to_string());
        let h = hash(&s);
        if h.starts_with("00000") {
            return (h, nonce);
        }
        nonce = nonce + 1;
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let mut counter = 0;
    let mut pwd = String::new();

    for _ in 0..8 {
        let (s, nonce) = find_next(input, counter);
        let str_bytes = s.as_bytes();
        pwd.push(str_bytes[5] as char);

        counter = nonce + 1;
    }
    pwd
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    let mut counter = 0;
    let mut pwd: HashMap<u32, char> = HashMap::new();

    loop {
        let (s, nonce) = find_next(input, counter);
        let s_bytes = s.as_bytes();
        let key: u32 = (s_bytes[5] as char).to_digit(10).unwrap_or(42); // Default valid, but out of range
        let val = s_bytes[6] as char;
        if key < 8 && !pwd.contains_key(&key) {
            pwd.insert(key, val);
        }

        if pwd.len() == 8 {
            let mut pwd_str = String::new();
            pwd_str.push_str(&pwd.get(&0).unwrap_or(&' ').to_string());

            let mut pwd_values = pwd.iter().collect::<Vec<_>>();

            pwd_values.sort();

            return pwd_values.iter().map(|(_, &value)| value).collect();
        }

        counter = nonce + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "abc";

    #[test]
    fn hasher() {
        assert_eq!(hash("hello world"), "5eb63bbbe01eeed093cb22bb8f5acdc3");
        assert_eq!(hash("test input"), "5eed650258ee02f6a77c87b748b764ec");
    }

    #[test]
    fn test_find_next() {
        let (hash, nonce) = find_next("abc", 3231928);
        assert_eq!(hash, "00000155f8105dff7f56ee10fa9b9abd");
        assert_eq!(nonce, 3231929);
    }

    #[ignore]
    #[test]
    fn test_part1() {
        // Slow hashing
        assert_eq!(part1(TEST_INPUT), "18f47a30");
    }

    #[ignore]
    #[test]
    fn test_part2() {
        // Slow hashing
        assert_eq!(part2(TEST_INPUT), "05ace8e3");
    }
}
