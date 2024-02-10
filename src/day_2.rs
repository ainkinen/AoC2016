use std::collections::HashMap;
use std::iter::Iterator;

use aoc_runner_derive::aoc;

type Coord = (i32, i32);

lazy_static! {
    static ref PART2_KEYS: HashMap<Coord, char> = {
        let mut map = HashMap::new();
        map.insert((0, 2), '1');
        map.insert((1, 1), '2');
        map.insert((1, 2), '3');
        map.insert((1, 3), '4');
        map.insert((2, 0), '5');
        map.insert((2, 1), '6');
        map.insert((2, 2), '7');
        map.insert((2, 3), '8');
        map.insert((2, 4), '9');
        map.insert((3, 1), 'A');
        map.insert((3, 2), 'B');
        map.insert((3, 3), 'C');
        map.insert((4, 2), 'D');
        map
    };
}

fn step(loc: &mut Coord, dir: char) {
    match dir {
        'U' => loc.0 -= 1,
        'D' => loc.0 += 1,
        'L' => loc.1 -= 1,
        'R' => loc.1 += 1,
        _ => panic!("Unknown direction {}", dir),
    }

    // clamp 0..=2
    loc.0 = loc.0.max(0).min(2);
    loc.1 = loc.1.max(0).min(2);
}

fn step_part_2(loc: &Coord, dir: char) -> Coord {
    let new_loc: Coord = match dir {
        'U' => (loc.0 - 1, loc.1),
        'D' => (loc.0 + 1, loc.1),
        'L' => (loc.0, loc.1 - 1),
        'R' => (loc.0, loc.1 + 1),
        _ => panic!("Unknown direction {}", dir),
    };

    if !PART2_KEYS.contains_key(&new_loc) {
        // Return old if new step is invalid
        return (loc.0, loc.1);
    }

    new_loc
}

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    let lines = input.lines();

    let mut loc: Coord = (1, 1);

    let mut digit_coords: Vec<Coord> = vec![];

    for l in lines {
        for dir in l.chars() {
            step(&mut loc, dir);
        }
        digit_coords.push(loc);
    }

    digit_coords
        .iter()
        .map(|c| c.0 * 3 + c.1 + 1) // Coord into digit
        .rev() // Reverse to enumerate as powers of 10
        .enumerate()
        .map(|(i, d)| i32::pow(10, i as u32) * d)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> String {
    let lines = input.lines();

    let mut loc: Coord = (2, 0);

    let mut digit_coords: Vec<Coord> = vec![];

    for l in lines {
        for dir in l.chars() {
            loc = step_part_2(&loc, dir);
        }
        digit_coords.push(loc);
    }

    digit_coords
        .iter()
        .map(|c| PART2_KEYS.get(c).expect("Invalid key"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "ULL\nRRDDD\nLURDL\nUUUUD";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1985);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "5DB3");
    }
}
