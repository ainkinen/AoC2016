use aoc_runner_derive::aoc;
use regex::Regex;

struct Disc {
    size: u32,
    at: u32,
}

fn parse_discs(input: &str) -> Vec<Disc> {
    lazy_static! {
        static ref NUMBER: Regex =
            Regex::new(r"Disc #(\d+) has (\d+) positions; at time=(\d+), it is at position (\d+).")
                .unwrap();
    }

    input
        .lines()
        .map(|l| NUMBER.captures(l).unwrap())
        .map(|cap| Disc {
            size: cap[2].parse().unwrap(),
            at: cap[4].parse().unwrap(),
        })
        .collect()
}

fn solve(discs: &Vec<Disc>) -> u32 {
    let mut time = 0_u32;
    loop {
        let lined = discs
            .iter()
            .enumerate()
            .all(|(idx, disc)| (disc.at + time + idx as u32 + 1) % disc.size == 0);

        if lined {
            return time;
        }

        time += 1;
    }
}

#[aoc(day15, part1)]
fn part1(input: &str) -> u32 {
    let discs = parse_discs(input);

    solve(&discs)
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u32 {
    let mut discs = parse_discs(input);
    discs.push(Disc { size: 11, at: 0 });

    solve(&discs)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 85);
    }
}
