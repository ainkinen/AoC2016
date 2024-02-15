use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn get_value(s: &str, registers: &HashMap<String, i32>) -> i32 {
    if let Some(value) = registers.get(s) {
        return *value;
    } else if let Ok(value) = s.parse::<i32>() {
        return value;
    } else {
        panic!("Unknown value: {}", s);
    }
}

fn run(ops: &mut Vec<String>, registers: &mut HashMap<String, i32>) {
    let mut pc = 0_i32;

    while pc >= 0 && pc < ops.len() as i32 {
        let cmd = ops.get(pc as usize).unwrap().clone();

        let (op, rest) = cmd.split_once(' ').unwrap();

        match op {
            "cpy" => {
                let (x, y) = rest.split_once(' ').unwrap();
                let value = get_value(x, &registers);
                registers.insert(y.to_owned(), value);
                pc += 1;
            }
            "inc" => {
                let value = get_value(rest, &registers);
                registers.insert(rest.to_owned(), value + 1);
                pc += 1;
            }
            "dec" => {
                let value = get_value(rest, &registers);
                registers.insert(rest.to_owned(), value - 1);
                pc += 1;
            }
            "jnz" => {
                let (x, y) = rest.split_once(' ').unwrap();
                let value = get_value(x, &registers);

                if value != 0 {
                    pc += get_value(y, &registers);
                } else {
                    pc += 1;
                }
            }
            "tgl" => {
                let value = get_value(rest, &registers);
                let target_idx = (pc + value) as usize;
                if target_idx < ops.len() {
                    let target = &ops[target_idx];
                    let parts: Vec<_> = target.split(' ').collect();
                    // println!("toggling target: {}, {:?}", target, parts);

                    ops[target_idx] = match (parts.len(), parts[0]) {
                        (2, "inc") => {
                            format!("dec {}", parts[1])
                        }
                        (2, _) => {
                            format!("inc {}", parts[1])
                        }
                        (3, "jnz") => {
                            format!("cpy {} {}", parts[1], parts[2])
                        }
                        (3, _) => {
                            format!("jnz {} {}", parts[1], parts[2])
                        }
                        _ => panic!("Unknown target cmd to toggle: {}", target),
                    };

                    // println!("new cmd: {}", ops[target_idx]);
                }

                pc += 1;
            }
            _ => panic!("Unknown op code: {}", op),
        }
    }
}

#[aoc(day23, part1)]
fn part1(input: &str) -> i32 {
    let mut ops: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    let mut registers = HashMap::from([
        ("a".to_owned(), 7),
        ("b".to_owned(), 0),
        ("c".to_owned(), 0),
        ("d".to_owned(), 0),
    ]);

    run(&mut ops, &mut registers);

    *registers.get("a").unwrap()
}

#[aoc(day23, part2)]
fn part2(input: &str) -> i32 {
    let mut ops: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    let mut registers = HashMap::from([
        ("a".to_owned(), 12),
        ("b".to_owned(), 0),
        ("c".to_owned(), 0),
        ("d".to_owned(), 0),
    ]);

    run(&mut ops, &mut registers);

    *registers.get("a").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }
}
