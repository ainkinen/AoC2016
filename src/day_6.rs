use std::cmp::Reverse;
use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Clone, Debug)]
struct CharCounter {
    map: HashMap<char, i32>,
}

impl CharCounter {
    fn new() -> CharCounter {
        CharCounter {
            map: HashMap::new(),
        }
    }

    fn count(self: &mut Self, c: char) {
        match self.map.get(&c) {
            Some(count) => self.map.insert(c, count + 1),
            None => self.map.insert(c, 1),
        };
    }

    fn most_frequent(self: &Self) -> char {
        let mut items = self.map.iter().collect::<Vec<_>>();
        items.sort_by_key(|(&char, &count)| (Reverse(count), char));
        *items.get(0).unwrap().0
    }

    fn least_frequent(self: &Self) -> char {
        let mut items = self.map.iter().collect::<Vec<_>>();
        items.sort_by_key(|(&char, &count)| (count, char));
        *items.get(0).unwrap().0
    }
}

fn count_chars(input: &str) -> Vec<CharCounter> {
    let length = input.lines().next().unwrap().len();
    let mut counters: Vec<CharCounter> = vec![CharCounter::new(); length];

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        for idx in 0..length {
            if let Some(counter) = counters.get_mut(idx) {
                counter.count(chars[idx]);
            }
        }
    }

    counters
}

#[aoc(day6, part1)]
fn part1(input: &str) -> String {
    let counters = count_chars(input);

    counters.iter().map(|c| c.most_frequent()).collect()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> String {
    let counters = count_chars(input);

    counters.iter().map(|c| c.least_frequent()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "easter");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "advent");
    }
}
