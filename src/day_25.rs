use std::collections::HashMap;
use std::iter;

use aoc_runner_derive::aoc;

fn get_value(s: &str, registers: &HashMap<&str, i32>) -> i32 {
    if let Some(value) = registers.get(s) {
        return *value;
    } else if let Ok(value) = s.parse::<i32>() {
        return value;
    } else {
        panic!("Unknown value: {}", s);
    }
}

fn run<'a>(
    ops: &Vec<&'a str>,
    registers: &mut HashMap<&'a str, i32>,
    signal_length: usize,
) -> Vec<i32> {
    let mut pc = 0_i32;

    let mut signal = Vec::new();

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
            "out" => {
                let value = get_value(rest, &registers);
                signal.push(value);
                pc += 1;
            }
            _ => panic!("Unknown op code: {}", op),
        }

        if signal.len() == signal_length {
            return signal;
        }
    }

    panic!("Failed to create full signal")
}

#[aoc(day25, part1)]
fn part1(input: &str) -> i32 {
    let ops: Vec<&str> = input.lines().collect();

    let signal_length = 20_usize;

    for i in 0..1_000 {
        let mut registers = HashMap::from([("a", i), ("b", 0), ("c", 0), ("d", 0)]);

        let signal = run(&ops, &mut registers, signal_length);

        // Lazy brute force
        let expected: Vec<i32> = iter::repeat(vec![0, 1])
            .flatten()
            .take(signal_length)
            .collect();
        if signal == expected {
            return i;
        }
    }

    /*
    Proper solution:

    The program instructions are split into two separate sections:
    1) One-time setup: Calculates a value into register 'd'
    2) Looping print: Outputs the value from register 'd' as binary

    For alternating bits the 'd' value should be 0b010101010101 = 2730.

    The loops in setup are a multiplication which gets added to the value of register 'a'.

    To solve the setup part:
    1) Extract multiplier 'a' from line 2.
    2) Extract multiplicand 'b' from line 3.

    The value needed for register 'a': 2730 - (a * b)
    */

    panic!("No result found under 1_000")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Printing part of the ops
    static TEST_INPUT: &str = "cpy d a
jnz 0 0
cpy a b
cpy 0 a
cpy 2 c
jnz b 2
jnz 1 6
dec b
dec c
jnz c -4
inc a
jnz 1 -7
cpy 2 b
jnz c 2
jnz 1 4
dec b
dec c
jnz 1 -4
jnz 0 0
out b
jnz a -19
jnz 1 -21";

    fn gen_signal(input: &str, d: i32, signal_length: usize) -> Vec<i32> {
        let ops: Vec<&str> = input.lines().collect();
        let mut registers = HashMap::from([("a", 0), ("b", 0), ("c", 0), ("d", d)]);

        run(&ops, &mut registers, signal_length)
    }

    #[test]
    fn test_part1() {
        // Tests that the printing part of the program prints the 'd' register value.
        // I.e. 2730 outputs the repeating pattern.
        let n = 1000;
        let expected: Vec<i32> = iter::repeat(vec![0, 1]).flatten().take(n).collect();
        assert_eq!(gen_signal(TEST_INPUT, 2730, n), expected);
    }
}
