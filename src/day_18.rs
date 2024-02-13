use aoc_runner_derive::aoc;

fn solve(input: &str, rows_needed: usize) -> usize {
    let mut rows: Vec<Vec<char>> = vec![input.chars().collect()];

    while rows.len() < rows_needed {
        let last_row = rows.last().unwrap();

        let new_row = (0..last_row.len() as i32)
            .map(|idx| {
                let (l, c, r) = (
                    last_row.get((idx - 1) as usize).unwrap_or(&'.'),
                    last_row.get(idx as usize).unwrap_or(&'.'),
                    last_row.get((idx + 1) as usize).unwrap_or(&'.'),
                );

                return match (l, c, r) {
                    ('^', '^', '.') => '^',
                    ('.', '^', '^') => '^',
                    ('^', '.', '.') => '^',
                    ('.', '.', '^') => '^',
                    _ => '.',
                };
            })
            .collect();

        rows.push(new_row)
    }

    rows.iter()
        .map(|r| r.iter().filter(|&&c| c == '.').count())
        .sum()
}

#[aoc(day18, part1)]
fn part1(input: &str) -> usize {
    solve(input, 40)
}

#[aoc(day18, part2)]
fn part2(input: &str) -> usize {
    solve(input, 400000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver() {
        assert_eq!(solve("..^^.", 3), 6);
        assert_eq!(solve(".^^.^.^^^^", 10), 38);
    }
}
