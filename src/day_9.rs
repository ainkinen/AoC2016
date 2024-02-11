use aoc_runner_derive::aoc;
use regex::Regex;

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
}

fn get_len(input: &str, recursive: bool) -> usize {
    let mut len = 0;

    let mut idx = 0;

    let chars = input.chars().collect::<Vec<_>>();

    while idx < input.len() {
        let char = chars[idx];
        if char == '(' {
            let marker_end = input[idx..].find(')').unwrap() + idx;
            let marker_part = &input[idx..=marker_end];

            let numbers = NUMBER
                .find_iter(marker_part)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let (repeated_input_len, repeat_times) = (numbers[0], numbers[1]);

            idx = marker_end + 1; // skip marker

            let repeated_len = if recursive {
                let repeated_part = &input[idx..idx + repeated_input_len];
                get_len(repeated_part, true)
            } else {
                repeated_input_len
            };

            idx += repeated_input_len; // skip repeated section
            len += repeat_times * repeated_len; // increase length by the repeated length
        } else {
            len += 1;
            idx += 1;
        }
    }

    len
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    get_len(input, false)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    get_len(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ADVENT"), 6);
        assert_eq!(part1("A(1x5)BC"), 7);
        assert_eq!(part1("(3x3)XYZ"), 9);
        assert_eq!(part1("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(part1("(6x1)(1x3)A"), 6);
        assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("ADVENT"), 6);
        assert_eq!(part2("(3x3)XYZ"), 9);
        assert_eq!(part2("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(
            part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
