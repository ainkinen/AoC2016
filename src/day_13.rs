use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::aoc;

struct Map {
    key: i32,
    tiles: HashMap<(i32, i32), bool>,
}

impl Map {
    fn new(key: i32) -> Map {
        Map {
            key,
            tiles: HashMap::new(),
        }
    }

    fn tile_is_open(self: &mut Self, loc: &(i32, i32)) -> bool {
        let (y, x) = loc;

        if *y < 0 || *x < 0 {
            return false;
        }

        if let Some(&is_open) = self.tiles.get(loc) {
            return is_open;
        }

        let value = x * x + 3 * x + 2 * x * y + y + y * y + self.key;
        let ones = format!("{:b}", value).chars().filter(|&c| c == '1').count();
        let is_open = ones % 2 == 0;

        is_open
    }
}

fn next_to(loc: (i32, i32)) -> [(i32, i32); 4] {
    let (y, x) = loc;
    [
        // (y - 1, x - 1),
        (y - 1, x),
        // (y - 1, x + 1),
        (y, x - 1),
        // (y, x)  current
        (y, x + 1),
        // (y + 1, x - 1),
        (y + 1, x),
        // (y + 1, x + 1),
    ]
}

fn bfs(map: &mut Map, goal: (i32, i32)) -> u32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut queue: VecDeque<(Vec<(i32, i32)>, i32, i32)> = VecDeque::from([(vec![], 1, 1)]);

    while let Some((path, y, x)) = queue.pop_front() {
        let loc = (y, x);

        // println!("{:?}", loc);
        if loc == goal {
            // println!("{:?}", path);
            return path.len() as u32; // Starting point counts as the last step
        }

        visited.insert(loc);

        next_to(loc)
            .into_iter()
            .filter(|l| !visited.contains(l))
            .filter(|l| map.tile_is_open(l))
            .for_each(|l| {
                let mut path = path.clone();
                path.push(loc);
                queue.push_back((path, l.0, l.1))
            });
    }

    panic!("Could not find route to end");
}

fn bfs2(map: &mut Map) -> u32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut heads: HashSet<(i32, i32)> = HashSet::from([(1, 1)]);

    for _ in 0..50 {
        let next_heads: HashSet<(i32, i32)> = heads
            .iter()
            .flat_map(|&loc| next_to(loc))
            .filter(|loc| !visited.contains(loc))
            .filter(|loc| map.tile_is_open(loc))
            .collect();

        visited.extend(next_heads.clone());

        heads = next_heads;
    }

    visited.len() as u32
}

fn run_part1(key: i32, goal: (i32, i32)) -> u32 {
    let mut map = Map::new(key);

    bfs(&mut map, goal)
}

#[aoc(day13, part1)]
fn part1(input: &str) -> u32 {
    let key = input.parse::<i32>().unwrap();
    run_part1(key, (39, 31))
}

#[aoc(day13, part2)]
fn part2(input: &str) -> u32 {
    let key = input.parse::<i32>().unwrap();
    let mut map = Map::new(key);
    bfs2(&mut map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(run_part1(10, (4, 7)), 11);
    }
}
