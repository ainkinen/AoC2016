use std::cmp::Reverse;
use std::collections::HashMap;
use std::str::FromStr;

use aoc_runner_derive::aoc;
use regex::Regex;

fn letter_counts(word: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    for c in word.chars().filter(|c| c.is_alphabetic()) {
        match counts.get(&c) {
            Some(count) => counts.insert(c, count + 1),
            None => counts.insert(c, 1),
        };
    }

    counts
}

#[derive(Debug)]
struct Room {
    name: String,
    sector: i32,
    checksum: String,
}

impl Room {
    fn calculate_checksum(self: &Self) -> String {
        let counts = letter_counts(&self.name);

        let mut items: Vec<_> = counts.iter().collect();

        items.sort_by_key(|(&letter, &count)| (Reverse(count), letter));

        items.iter().take(5).map(|(&letter, _)| letter).collect()
    }

    fn is_real(self: &Self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    fn decrypt_name(self: &Self) -> String {
        self.name
            .chars()
            .map(|char| {
                if char.is_alphabetic() {
                    let alpha_idx = (char as i32 - 97 + self.sector) % 26 + 97;
                    return char::from(alpha_idx as u8);
                }
                char
            })
            .collect()
    }
}

impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.+)-(\d+)\[([a-z]+)]").unwrap();
        }

        if let Some(captures) = RE.captures(s) {
            return Ok(Room {
                name: captures[1].to_owned(),
                sector: captures[2].parse().expect("Failed to unwrap sector number"),
                checksum: captures[3].to_owned(),
            });
        }

        Err(())
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {
    let real_rooms = input
        .lines()
        .map(|l| Room::from_str(l).unwrap())
        .filter(|r| r.is_real());

    real_rooms.map(|r| r.sector).sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    let real_rooms = input
        .lines()
        .map(|l| Room::from_str(l).unwrap())
        .filter(|r| r.is_real());

    for room in real_rooms {
        if room.decrypt_name() == "northpole-object-storage" {
            return room.sector;
        }
    }

    panic!("North Pole object storage not found!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_counts() {
        assert_eq!(
            letter_counts("aaaaa-bbb-z-y-x"),
            HashMap::from([('a', 5), ('b', 3), ('z', 1), ('y', 1), ('x', 1),])
        );
    }

    static TEST_INPUT: &str = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1514);
    }
}
