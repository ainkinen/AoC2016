use std::collections::VecDeque;

use aoc_runner_derive::aoc;

#[allow(dead_code)]
// #[aoc(day19, part1, brute_force)]
fn part1_brute_force(input: &str) -> i32 {
    let num_elves: i32 = input.parse().unwrap();

    let mut elves: VecDeque<_> = (1..=num_elves).collect();

    let mut turn_idx = 0;

    while elves.len() > 1 {
        if turn_idx < elves.len() - 2 {
            elves.remove(turn_idx + 1);
            turn_idx += 1;
            continue;
        } else if turn_idx == elves.len() - 2 {
            elves.remove(turn_idx + 1);
            turn_idx = 0;
            continue;
        } else if turn_idx == elves.len() - 1 {
            elves.remove(0);
            turn_idx = 0;
            continue;
        }
    }

    *elves.front().unwrap()
}

#[aoc(day19, part1)]
fn part1(input: &str) -> i32 {
    let num_elves: i32 = input.parse().unwrap();

    let mut ring: VecDeque<i32> = (1..=num_elves).collect();

    while ring.len() > 1 {
        let elf_in_turn = ring.pop_front().unwrap();
        ring.push_back(elf_in_turn); // Stealer goes to the back
        ring.pop_front(); // Eliminated
    }

    ring[0]
}

#[aoc(day19, part2)]
fn part2(input: &str) -> i32 {
    let num_elves: i32 = input.parse().unwrap();

    // Two dequeues to keep track of halves of the ring
    let mut half1: VecDeque<i32> = (1..=(num_elves + 1) / 2).collect();
    let mut half2: VecDeque<i32> = (((num_elves + 1) / 2 + 1)..=num_elves).collect();

    loop {
        // The elf to remove is at the back of the (longer) halve
        if half2.len() >= half1.len() {
            half2.pop_front();
            if half2.is_empty() {
                // second half is empty, the last elf must the only member on the other half
                return half1[0];
            }
        } else {
            half1.pop_back();
        }

        // Rotate the halves for the next turn
        half1.push_back(half2.pop_front().unwrap());
        half2.push_back(half1.pop_front().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}
