use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use regex::Regex;

fn scramble(cmd: &str, password: &mut VecDeque<char>) {
    lazy_static! {
        static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
        static ref LETTER_SWAP: Regex =
            Regex::new(r"swap letter (\p{L}) with letter (\p{L})").unwrap();
        static ref ROTATE_LR: Regex = Regex::new(r"rotate (left|right) (\d+) steps?").unwrap();
    }

    if cmd.starts_with("swap position") {
        let numbers: Vec<usize> = NUMBER
            .find_iter(cmd)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let (x, y) = (numbers[0], numbers[1]);

        (password[x], password[y]) = (password[y], password[x]);

        return;
    }

    if cmd.starts_with("swap letter") {
        let captures = LETTER_SWAP.captures(cmd).unwrap();
        let (char_x, char_y) = (
            captures[1].chars().next().unwrap(),
            captures[2].chars().next().unwrap(),
        );

        let (x, y) = (
            password.iter().position(|&c| c == char_x).unwrap(),
            password.iter().position(|&c| c == char_y).unwrap(),
        );
        (password[x], password[y]) = (password[y], password[x]);

        return;
    }

    if cmd.starts_with("reverse") {
        let numbers: Vec<usize> = NUMBER
            .find_iter(cmd)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let (x, y) = (numbers[0], numbers[1]);

        let mut section = password.make_contiguous()[x..=y].to_vec();
        section.reverse();

        for (idx, char) in (x..=y).zip(section) {
            password[idx] = char;
        }

        return;
    }

    if cmd.starts_with("rotate based") {
        let key = cmd.chars().last().unwrap();

        let idx = password.iter().position(|&c| c == key).unwrap();

        let steps = if idx >= 4 { 1 + idx + 1 } else { 1 + idx };
        password.rotate_right(steps % password.len());

        return;
    }

    if cmd.starts_with("rotate") {
        let captures = ROTATE_LR.captures(cmd).unwrap();
        let steps: usize = captures[2].parse().unwrap();

        if captures[1] == *"left" {
            password.rotate_left(steps);
        } else {
            password.rotate_right(steps);
        }
        return;
    }

    if cmd.starts_with("move") {
        let numbers: Vec<usize> = NUMBER
            .find_iter(cmd)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let char = password.remove(numbers[0]).unwrap();
        password.insert(numbers[1], char);

        return;
    }

    panic!("Unknown command: {}", cmd);
}

fn part1_solver(input: &str, password: &str) -> String {
    let mut password: VecDeque<char> = password.chars().collect();

    for cmd in input.lines() {
        scramble(cmd, &mut password);
    }

    password.iter().collect()
}

#[aoc(day21, part1)]
fn part1(input: &str) -> String {
    part1_solver(input, "abcdefgh")
}

fn part2_solver(input: &str, password: &str) -> String {
    let orig_password: VecDeque<char> = password.chars().collect();

    // Lazy brute force
    for perm in orig_password.iter().permutations(password.len()) {
        let mut password: VecDeque<char> = perm.iter().map(|&&c| c).collect();

        for cmd in input.lines() {
            scramble(cmd, &mut password);
        }

        if password == orig_password {
            return perm.iter().map(|&&c| c).collect();
        }
    }

    panic!("Could not descramble")
}

#[aoc(day21, part2)]
fn part2(input: &str) -> String {
    part2_solver(input, "fbgdceah")
}

#[cfg(test)]
mod tests {
    use crate::vec_of_strings;

    use super::*;

    #[test]
    fn test_scramble() {
        let mut pwd = VecDeque::from(vec!['a', 'b', 'c', 'd', 'e']);

        scramble("swap position 4 with position 0", &mut pwd);
        assert_eq!(pwd.iter().collect::<String>(), "ebcda");

        scramble("swap letter d with letter b", &mut pwd);
        assert_eq!(pwd.iter().collect::<String>(), "edcba");

        scramble("reverse positions 0 through 4", &mut pwd);
        assert_eq!(pwd.iter().collect::<String>(), "abcde");

        scramble("rotate left 1 step", &mut pwd);
        assert_eq!(pwd.iter().collect::<String>(), "bcdea");

        scramble("move position 1 to position 4", &mut pwd);
        assert_eq!(pwd.iter().collect::<String>(), "bdeac");

        scramble("move position 3 to position 0", &mut pwd);
        assert_eq!(pwd.iter().collect::<String>(), "abdec");

        scramble("rotate based on position of letter b", &mut pwd);
        assert_eq!(pwd.iter().collect::<String>(), "ecabd");

        scramble("rotate based on position of letter d", &mut pwd);
        assert_eq!(pwd.iter().collect::<String>(), "decab");
    }

    static TEST_INPUT: &str = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    // #[ignore]
    #[test]
    fn test_part1() {
        assert_eq!(part1_solver(TEST_INPUT, "abcde"), "decab");
    }

    #[test]
    fn test_part2() {
        let result = part2_solver(TEST_INPUT, "decab");
        assert!(vec_of_strings!["abcde", "deabc"].contains(&result));
    }
}
