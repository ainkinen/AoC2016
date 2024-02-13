use aoc_runner_derive::aoc;

fn optimal_moves(items: u32) -> u32 {
    // Optimal moves for moving n items up a floor
    2 * (items - 1) - 1
}

fn solve(items_per_floor: [u32; 3]) -> u32 {
    let [on_floor_1, on_floor_2, on_floor_3] = items_per_floor;

    optimal_moves(on_floor_1)
        + optimal_moves(on_floor_1 + on_floor_2)
        + optimal_moves(on_floor_1 + on_floor_2 + on_floor_3)
}

fn parse_starting_counts(input: &str) -> [u32; 3] {
    let counts: Vec<u32> = input
        .lines()
        .take(3)
        .map(|l| l.chars().filter(|&c| c == ',').count() as u32)
        .map(|v| v + 1)
        .collect();

    [counts[0], counts[1], counts[2]]
}

#[aoc(day11, part1)]
fn part1(input: &str) -> u32 {
    let starting_counts = parse_starting_counts(input);
    solve(starting_counts)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> u32 {
    let starting_counts = parse_starting_counts(input);
    let with_extra_items = [
        starting_counts[0] + 4,
        starting_counts[1],
        starting_counts[2],
    ];
    solve(with_extra_items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve([2, 1, 1]), 9);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve([6, 1, 1]), 33);
    }
}
