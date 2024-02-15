use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::aoc;
use itertools::Itertools;

type Coord = (i32, i32);
type Grid = HashSet<Coord>;

fn bfs(from: Coord, to: Coord, grid: &Grid) -> u32 {
    let mut heads: VecDeque<(Coord, u32)> = VecDeque::from([(from, 0)]);
    let mut visited: HashSet<Coord> = HashSet::new();

    while let Some(((y, x), steps)) = heads.pop_front() {
        if (y, x) == to {
            return steps;
        }

        vec![(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
            .iter()
            .filter(|&loc| grid.contains(loc))
            .for_each(|mov| {
                if visited.insert(*mov) {
                    heads.push_back((*mov, steps + 1));
                };
            });
    }

    panic!("Could not find route")
}

fn solver(input: &str, is_part2: bool) -> u32 {
    let stops: HashMap<char, Coord> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c.is_digit(10) {
                    Some((c, (y as i32, x as i32)))
                } else {
                    None
                }
            })
        })
        .collect();

    let stop_pairs: Vec<[&char; 2]> = stops
        .keys()
        .combinations(2)
        .filter_map(|v| v.try_into().ok())
        .collect();

    let grid: HashSet<Coord> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    None
                } else {
                    Some((y as i32, x as i32))
                }
            })
        })
        .collect();

    let mut path_lengths: HashMap<(char, char), u32> = HashMap::new();
    for [&from_c, &to_c] in stop_pairs {
        let l = bfs(stops[&from_c], stops[&to_c], &grid);
        // Save both directions for easy access
        path_lengths.insert((from_c, to_c), l);
        path_lengths.insert((to_c, from_c), l);
    }

    // println!("path lengths: {:?}", path_lengths);

    let route_len = |route: Vec<&char>| -> u32 {
        // println!("route: {:?}", route);
        route
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|(&a, &b)| {
                // println!("pair: {:?}", (a, b));
                path_lengths[&(*a, *b)]
            })
            .sum()
    };

    let min_route = stops
        .keys()
        .permutations(stops.len())
        .filter(|route| *route[0] == '0')
        .map(|route| {
            if is_part2 {
                let mut r = route.clone();
                r.push(&'0');
                r
            } else {
                route
            }
        })
        .map(route_len)
        .min();

    min_route.unwrap()
}

#[aoc(day24, part1)]
fn part1(input: &str) -> u32 {
    solver(input, false)
}

#[aoc(day24, part2)]
fn part2(input: &str) -> u32 {
    solver(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 20);
    }
}
