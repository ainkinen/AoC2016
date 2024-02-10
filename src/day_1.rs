use std::collections::HashSet;

use aoc_runner_derive::aoc;

type Dir = i8; // N = 0, E = 1, S = 2, W = 3

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Coord {
    y: i32,
    x: i32,
}

fn turn(d: Dir, c: char) -> Dir {
    match c {
        'R' => return (d + 1 + 4) % 4,
        'L' => return (d - 1 + 4) % 4,
        _ => panic!("Unknown turn {c}"),
    }
}

fn step(loc: &Coord, d: Dir, i: i32) -> Coord {
    match d {
        0 => Coord {
            y: loc.y - i,
            x: loc.x,
        }, // N
        1 => Coord {
            x: loc.x + i,
            y: loc.y,
        }, // E
        2 => Coord {
            y: loc.y + i,
            x: loc.x,
        }, // S
        3 => Coord {
            x: loc.x - i,
            y: loc.y,
        }, // W
        _ => panic!("Invalid direction: {d}"),
    }
}

#[aoc(day1, part1)]
fn part1(command_str: &str) -> i32 {
    let steps = command_str.split(", ");

    let mut d: Dir = 0;
    let mut loc = Coord { y: 0, x: 0 };
    for s in steps {
        let turn_dir = s.chars().nth(0).expect("Unknown sequence command.");
        let count: i32 = s[1..].parse().expect("Could not parse step count.");

        d = turn(d, turn_dir);
        loc = step(&loc, d, count);
    }
    loc.y.abs() + loc.x.abs()
}

#[aoc(day1, part2)]
fn part2(command_str: &str) -> i32 {
    let steps = command_str.split(", ");

    let mut d: Dir = 0;
    let loc = Coord { y: 0, x: 0 };

    let mut seen_locs = HashSet::new();

    for s in steps.cycle() {
        let turn_dir = s.chars().nth(0).expect("Unknown sequence command.");
        let count: i32 = s[1..].parse().expect("Could not parse step count.");

        d = turn(d, turn_dir);
        let loc = step(&loc, d, count);

        if !seen_locs.insert(loc) {
            return loc.y.abs() + loc.x.abs();
        }
    }

    panic!("Could not find a double location!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_right() {
        assert_eq!(turn(0, 'R'), 1);
        assert_eq!(turn(1, 'R'), 2);
        assert_eq!(turn(2, 'R'), 3);
        assert_eq!(turn(3, 'R'), 0);
    }

    #[test]
    fn turn_left() {
        assert_eq!(turn(0, 'L'), 3);
        assert_eq!(turn(1, 'L'), 0);
        assert_eq!(turn(2, 'L'), 1);
        assert_eq!(turn(3, 'L'), 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("R2, L3"), 5);
        assert_eq!(part1("R2, R2, R2"), 2);
        assert_eq!(part1("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("R8, R4, R4, R8"), 8);
    }
}
