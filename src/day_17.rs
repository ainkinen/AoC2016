use aoc_runner_derive::aoc;
use md5::{Digest, Md5};
use std::collections::VecDeque;

type Coord = (i32, i32);

fn hash_path(passcode: &str, path: &str) -> [char; 4] {
    let mut hasher = Md5::new();
    hasher.update(passcode);
    hasher.update(path);
    let result = hasher.finalize();
    hex::encode(result)
        .chars()
        .take(4)
        .collect::<Vec<char>>()
        .try_into()
        .unwrap()
}

fn on_grid(loc: &Coord) -> bool {
    let (y, x) = loc;
    (0..4).contains(y) && (0..4).contains(x)
}

fn door_open(hash: &[char; 4], dir: &char) -> bool {
    let keys = "bcdef";

    match dir {
        'U' => return keys.contains(hash[0]),
        'D' => return keys.contains(hash[1]),
        'L' => return keys.contains(hash[2]),
        'R' => return keys.contains(hash[3]),
        _ => panic!("Unknown direction {}", dir),
    }
}

fn open_doors(passcode: &str, path: &str, loc: Coord) -> Vec<(char, Coord)> {
    let (y, x) = loc;
    let hash = hash_path(passcode, path);

    vec![
        ('U', (y - 1, x)),
        ('L', (y, x - 1)),
        ('R', (y, x + 1)),
        ('D', (y + 1, x)),
    ]
    .into_iter()
    .filter(|(_, loc)| on_grid(loc)) // on grid
    .filter(|(dir, _)| door_open(&hash, dir))
    .collect()
}

#[aoc(day17, part1)]
fn part1(passcode: &str) -> String {
    let goal = (3, 3);
    let mut heads: VecDeque<(Coord, String)> = VecDeque::from(vec![((0, 0), "".to_owned())]);

    while let Some((loc, path)) = heads.pop_front() {
        if loc == goal {
            return path;
        }

        for (dir, new_loc) in open_doors(&passcode, &path, loc) {
            let mut new_path = path.clone();
            new_path.push(dir);
            heads.push_back((new_loc, new_path));
        }
    }

    panic!("Failed to find a path")
}

#[aoc(day17, part2)]
fn part2(passcode: &str) -> usize {
    let mut longest_found = 0_usize;

    let goal = (3, 3);
    let mut heads: VecDeque<(Coord, String)> = VecDeque::from(vec![((0, 0), "".to_owned())]);

    while let Some((loc, path)) = heads.pop_front() {
        if loc == goal {
            longest_found = longest_found.max(path.len());
            continue;
        }

        for (dir, new_loc) in open_doors(&passcode, &path, loc) {
            let mut new_path = path.clone();
            new_path.push(dir);
            heads.push_back((new_loc, new_path));
        }
    }

    longest_found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_path() {
        assert_eq!(hash_path("hijkl", ""), ['c', 'e', 'd', '9']);
        assert_eq!(hash_path("hijklD", ""), ['f', '2', 'b', 'c']);
        assert_eq!(hash_path("hijklDR", ""), ['5', '7', '4', '5']);
        assert_eq!(hash_path("hijklDU", ""), ['5', '2', '8', 'e']);
    }

    #[test]
    fn test_door_open() {
        assert_eq!(door_open(&['c', 'e', 'd', '9'], &'U'), true);
        assert_eq!(door_open(&['c', 'e', 'd', '9'], &'D'), true);
        assert_eq!(door_open(&['c', 'e', 'd', '9'], &'L'), true);
        assert_eq!(door_open(&['c', 'e', 'd', '9'], &'R'), false);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("ihgpwlah"), "DDRRRD");
        assert_eq!(part1("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[ignore]
    #[test]
    fn test_part2() {
        // slow
        assert_eq!(part2("ihgpwlah"), 370);
        assert_eq!(part2("kglvqrro"), 492);
        assert_eq!(part2("ulqzkmiv"), 830);
    }
}
