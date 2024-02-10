use aoc_runner_derive::aoc;

fn is_possible(a: i32, b: i32, c: i32) -> bool {
    (a + b > c) && (a + c > b) && (b + c > a)
}

fn parse_line(line: &str) -> (i32, i32, i32) {
    if let [Ok(a), Ok(b), Ok(c)] = line
        .split_whitespace()
        .map(|d| d.parse::<i32>())
        .collect::<Vec<_>>()[..]
    {
        return (a, b, c);
    }

    panic!("Failed to parse line: {}", line);
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| parse_line(l))
        .filter(|(a, b, c)| is_possible(*a, *b, *c))
        .collect::<Vec<_>>()
        .len() as i32
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    let lines: Vec<(i32, i32, i32)> = input.lines().map(|l| parse_line(l)).collect();

    let transposed: Vec<(i32, i32, i32)> = lines
        .chunks(3)
        .flat_map(|chunk| {
            [
                (chunk[0].0, chunk[1].0, chunk[2].0),
                (chunk[0].1, chunk[1].1, chunk[2].1),
                (chunk[0].2, chunk[1].2, chunk[2].2),
            ]
        })
        .collect();

    transposed
        .iter()
        .filter(|(a, b, c)| is_possible(*a, *b, *c))
        .collect::<Vec<_>>()
        .len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str =
        "101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 603";

    #[test]
    fn test_is_possible() {
        assert!(is_possible(2, 3, 4));

        assert!(!is_possible(5, 10, 15));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}
