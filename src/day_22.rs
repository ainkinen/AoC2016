use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use regex::Regex;

type Coord = (i32, i32);
type Grid = HashSet<Coord>;

#[derive(Debug, Copy, Clone)]
struct Node {
    loc: Coord,
    size: i32,
    used: i32,
}

impl Node {
    fn avail(self: &Self) -> i32 {
        self.size - self.used
    }

    fn is_empty(self: &Self) -> bool {
        self.used == 0
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref DF_LINE: Regex =
                Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T.*").unwrap();
        }

        let Some(captures) = DF_LINE.captures(s) else {
            return Err(());
        };

        let x = captures[1].parse().map_err(|_| ())?;
        let y = captures[2].parse().map_err(|_| ())?;
        let size = captures[3].parse().map_err(|_| ())?;
        let used = captures[4].parse().map_err(|_| ())?;

        Ok(Node {
            loc: (y, x),
            size,
            used,
        })
    }
}

#[aoc(day22, part1)]
fn part1(input: &str) -> usize {
    let nodes: Vec<_> = input
        .lines()
        .filter_map(|l| l.parse::<Node>().ok())
        .collect();

    nodes
        .iter()
        .permutations(2)
        .filter(|pair| {
            let (a, b) = (pair[0], pair[1]);

            !a.is_empty() && a.used <= b.avail()
        })
        .count()
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct BfsState {
    empty_node_at: Coord,
    wanted_data_at: Coord,
    steps: u32,
}

fn possible_moves(current_loc: &Coord, grid: &Grid) -> Vec<Coord> {
    let (y, x) = *current_loc;
    vec![(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
        .into_iter()
        .filter(|loc| grid.contains(loc))
        .collect()
}

#[aoc(day22, part2)]
fn part2(input: &str) -> u32 {
    let nodes: Vec<_> = input
        .lines()
        .filter_map(|l| l.parse::<Node>().ok())
        .collect();

    let (max_y, max_x) = nodes.iter().fold((0, 0), |acc, node| {
        (acc.0.max(node.loc.0), acc.1.max(node.loc.1))
    });
    let _ = max_y; // Kept for drawing

    let empty_node = nodes.iter().find(|&n| n.used == 0).unwrap();

    let grid: HashSet<Coord> = nodes
        .iter()
        .filter(|n| n.used <= empty_node.size)
        .map(|&n| n.loc)
        .collect();

    // Draw grid for manual solving
    // println!("Grid:");
    // for y in 0..=max_y {
    //     let row: String = (0..=max_x)
    //         .map(|x| {
    //             let loc = (y, x);
    //             if loc == empty_node.loc {
    //                 return '0'; // empty node
    //             } else if loc == (0, 0) {
    //                 return 'G'; // goal
    //             } else if loc == (0, max_x) {
    //                 return 'D'; // wanted data
    //             } else if grid.contains(&loc) {
    //                 return '.'; // movable node
    //             }
    //             'x' // immovable data node
    //         })
    //         .collect();
    //     println!("{}", row);
    // };

    // bfs
    let mut states: VecDeque<BfsState> = VecDeque::from(vec![BfsState {
        empty_node_at: empty_node.loc,
        wanted_data_at: (0, max_x),
        steps: 0,
    }]);

    let mut seen: HashSet<(Coord, Coord)> = HashSet::from([(empty_node.loc, (0, max_x))]);
    let goal: Coord = (0, 0);

    // bfs
    while !states.is_empty() {
        let BfsState {
            empty_node_at,
            wanted_data_at,
            steps,
        } = states.pop_front().unwrap();

        if wanted_data_at == goal {
            return steps;
        }

        possible_moves(&empty_node_at, &grid)
            .into_iter()
            .map(|next_loc| {
                let new_state = BfsState {
                    empty_node_at: next_loc,
                    wanted_data_at: if next_loc == wanted_data_at {
                        empty_node_at
                    } else {
                        wanted_data_at
                    },
                    steps: steps + 1,
                };
                (next_loc, new_state)
            })
            .for_each(|(next_loc, s)| {
                if seen.insert((next_loc, s.wanted_data_at)) {
                    states.push_back(s);
                }
            });
    }

    panic!("Could not find a path")
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 7);
    }
}
