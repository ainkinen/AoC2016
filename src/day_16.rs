use aoc_runner_derive::aoc;

fn fold(input: &str, length: usize) -> String {
    let mut res: Vec<_> = input.chars().collect();
    while res.len() <= length {
        let mut copy = res.clone();
        copy.reverse();
        copy = copy
            .iter()
            .map(|&c| if c == '1' { '0' } else { '1' })
            .collect();
        res.push('0');
        res.append(&mut copy);
    }

    res.iter().take(length).collect()
}

fn checksum(s: &str) -> String {
    let mut checksum = s.chars().collect::<Vec<char>>();
    loop {
        let chunks = checksum.chunks(2).collect::<Vec<_>>();

        let condensed: Vec<char> = chunks
            .iter()
            .map(|chunk| if chunk[0] == chunk[1] { '1' } else { '0' })
            .collect();

        checksum = condensed;

        if checksum.len() % 2 == 1 {
            return checksum.iter().collect::<String>();
        }
    }
}

fn solver(input: &str, length: usize) -> String {
    let fill = fold(input, length);
    checksum(&fill)
}

#[aoc(day16, part1)]
fn part1_real(input: &str) -> String {
    solver(input, 272)
}

#[aoc(day16, part2)]
fn part2(input: &str) -> String {
    solver(input, 35651584)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold() {
        assert_eq!(fold("1", 3), "100");
        assert_eq!(fold("0", 3), "001");
        assert_eq!(fold("11111", 11), "11111000000");
        assert_eq!(fold("111100001010", 25), "1111000010100101011110000");
    }

    static TEST_INPUT: &str = "10000";

    #[test]
    fn test_solver() {
        assert_eq!(solver(TEST_INPUT, 20), "01100");
    }
}
