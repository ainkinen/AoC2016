use aoc_runner_derive::aoc;
use std::collections::HashSet;

fn is_abba(slice: &str) -> bool {
    let mut in_hypernet = false;
    let mut valid = false;
    let slice: Vec<_> = slice.chars().collect();

    for window in slice.windows(4) {
        if window[0] == '[' || window[0] == ']' {
            in_hypernet = !in_hypernet;
        }

        let abba: bool = window[0] != window[1] && window[0] == window[3] && window[1] == window[2];

        if abba {
            if in_hypernet {
                return false;
            } else {
                valid = true;
            }
        }
    }

    valid
}

fn find_abas_and_babs(slice: &Vec<char>) -> (HashSet<&[char]>, HashSet<&[char]>) {
    let mut in_hypernet = false;
    let mut abas: HashSet<&[char]> = HashSet::new();
    let mut babs: HashSet<&[char]> = HashSet::new();

    for window in slice.windows(3) {
        if window[0] == '[' || window[0] == ']' {
            in_hypernet = !in_hypernet;
        }

        let ababab: bool = window[0] != window[1] && window[0] == window[2];

        if ababab {
            if !in_hypernet {
                _ = abas.insert(window);
            } else {
                _ = babs.insert(window);
            }
        }
    }
    (abas, babs)
}

fn is_aba(slice: &str) -> bool {
    let slice: Vec<_> = slice.chars().collect();

    let (abas, babs) = find_abas_and_babs(&slice);

    for aba in abas {
        // Check if matching bab exists
        let bab = [aba[1], aba[0], aba[1]];
        if babs.contains(&bab as &[char]) {
            return true;
        }
    }

    false
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    input.lines().filter(|l| is_abba(l)).count()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    input.lines().filter(|l| is_aba(l)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abba() {
        assert!(is_abba("abba[mnop]qrst"));
        assert!(!is_abba("abcd[bddb]xyyx"));
        assert!(!is_abba("aaaa[qwer]tyui"));
        assert!(is_abba("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_is_aba() {
        assert!(is_aba("aba[bab]xyz"));
        assert!(!is_aba("xyx[xyx]xyx"));
        assert!(is_aba("aaa[kek]eke"));
        assert!(is_aba("zazbz[bzb]cdb"));
    }

    static TEST_INPUT: &str =
        "abba[mnop]qrst\nabcd[bddb]xyyx\naaaa[qwer]tyui\nioxxoj[asdfgh]zxcvbn";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    static TEST_INPUT_2: &str = "aba[bab]xyz\nxyx[xyx]xyx\naaa[kek]eke\nzazbz[bzb]cdb";

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 3);
    }
}
