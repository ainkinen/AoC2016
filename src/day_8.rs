use aoc_runner_derive::aoc;
use regex::Regex;

type Screen = Vec<Vec<char>>;

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
}

fn draw_rect(screen: &mut Screen, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            screen[y][x] = '#';
        }
    }
}

fn rotate_row(screen: &mut Screen, row: usize, by: usize) {
    screen[row].rotate_right(by)
}

fn rotate_column(screen: &mut Screen, column: usize, by: usize) {
    let mut extracted_column = screen.iter().map(|row| row[column]).collect::<Vec<_>>();

    extracted_column.rotate_right(by);

    for (row, &value) in extracted_column.iter().enumerate() {
        screen[row][column] = value;
    }
}

fn update_screen(screen: &mut Screen, cmd: &str) {
    let numbers = NUMBER
        .find_iter(cmd)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(numbers.len(), 2);

    if cmd.starts_with("rect") {
        draw_rect(screen, numbers[0], numbers[1]);
    } else if cmd.starts_with("rotate row") {
        rotate_row(screen, numbers[0], numbers[1]);
    } else if cmd.starts_with("rotate column") {
        rotate_column(screen, numbers[0], numbers[1]);
    } else {
        panic!("Unknown command: {}", cmd);
    };
}

fn swipe_card(input: &str) -> Screen {
    let mut screen: Screen = vec![vec!['.'; 50]; 6];

    for line in input.lines() {
        update_screen(&mut screen, line);
    }

    screen
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let screen = swipe_card(input);

    screen
        .iter()
        .map(|row| row.iter().filter(|&&pixel| pixel == '#').count())
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i32 {
    let screen = swipe_card(input);

    for row in screen {
        println!("{:?}", row.iter().collect::<String>());
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str =
        "rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4\nrotate column x=1 by 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6);
    }
}
