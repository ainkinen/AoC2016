use aoc_runner_derive::aoc;
use std::collections::HashMap;

fn get_value(s: &str, registers: &HashMap<&str, i32>) -> i32 {
    if let Some(value) = registers.get(s) {
        return *value;
    } else if let Ok(value) = s.parse::<i32>() {
        return value;
    } else {
        panic!("Unknown value: {}", s);
    }
}

fn run<'a>(ops: Vec<&'a str>, registers: &mut HashMap<&'a str, i32>) {
    let mut pc = 0_i32;

    while let Some(cmd) = ops.get(pc as usize) {
        let (op, rest) = cmd.split_once(' ').unwrap();

        match op {
            "cpy" => {
                let (x, y) = rest.split_once(' ').unwrap();
                let value = get_value(x, &registers);
                registers.insert(y, value);
                pc += 1;
            }
            "inc" => {
                let value = get_value(rest, &registers);
                registers.insert(rest, value + 1);
                pc += 1;
            }
            "dec" => {
                let value = get_value(rest, &registers);
                registers.insert(rest, value - 1);
                pc += 1;
            }
            "jnz" => {
                let (x, y) = rest.split_once(' ').unwrap();
                let value = get_value(x, &registers);

                if value != 0 {
                    pc += y.parse::<i32>().unwrap();
                } else {
                    pc += 1;
                }
            }
            _ => panic!("Unknown op code: {}", op),
        }
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    let ops: Vec<&str> = input.lines().collect();
    let mut registers = HashMap::from([("a", 0), ("b", 0), ("c", 0), ("d", 0)]);

    run(ops, &mut registers);

    *registers.get("a").unwrap()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    let ops: Vec<&str> = input.lines().collect();
    let mut registers = HashMap::from([("a", 0), ("b", 0), ("c", 1), ("d", 0)]);

    run(ops, &mut registers);

    *registers.get("a").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 42);
    }
}
