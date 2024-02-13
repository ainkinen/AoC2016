use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use aoc_runner_derive::aoc;
use regex::Regex;

#[derive(Debug)]
struct Give {
    to: String,
    chip: u32,
}

impl FromStr for Give {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref VALUE: Regex = Regex::new(r"value (\d+) goes to (bot \d+)").unwrap();
        }

        let matches = VALUE.captures(s).unwrap();

        Ok(Give {
            chip: matches[1].parse().unwrap(),
            to: matches[2].to_string(),
        })
    }
}

struct Bot {
    id: String,
    chip_in_hand: Option<u32>,
    low_dest: String,
    high_dest: String,
}

impl Bot {
    fn give_chip(self: &mut Self, given_chip: u32) -> Vec<Give> {
        let mut gives: Vec<Give> = Vec::new();
        match self.chip_in_hand {
            Some(chip_in_hand) => {
                let (low, high) = (chip_in_hand.min(given_chip), chip_in_hand.max(given_chip));

                let give_low = Give {
                    to: self.low_dest.clone(),
                    chip: low,
                };
                let give_high = Give {
                    to: self.high_dest.clone(),
                    chip: high,
                };
                gives.push(give_low);
                gives.push(give_high);
                self.chip_in_hand = None;
            }
            None => {
                self.chip_in_hand = Some(given_chip);
            }
        };

        gives
    }
}

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BOT: Regex = Regex::new(
                r"(bot \d+) gives low to ((?:bot|output) \d+) and high to ((?:bot|output) \d+)"
            )
            .unwrap();
        }

        let matches = BOT.captures(s).unwrap();

        Ok(Bot {
            id: matches[1].to_string(),
            chip_in_hand: None,
            low_dest: matches[2].to_string(),
            high_dest: matches[3].to_string(),
        })
    }
}

fn build_bots(bot_inputs: Vec<&str>) -> HashMap<String, Bot> {
    bot_inputs
        .iter()
        .map(|i| i.parse::<Bot>().unwrap())
        .map(|b| (b.id.clone(), b))
        .collect()
}

fn build_gives(value_inputs: Vec<&str>) -> VecDeque<Give> {
    value_inputs
        .iter()
        .map(|i| i.parse::<Give>().unwrap())
        .collect()
}

fn find_bot(input: &str, chip1: u32, chip2: u32) -> (Option<String>, HashMap<String, u32>) {
    let (value_inputs, bot_inputs): (_, Vec<_>) =
        input.lines().partition(|l| l.starts_with("value"));

    let mut bots = build_bots(bot_inputs);
    let mut gives = build_gives(value_inputs);
    let mut outputs: HashMap<String, u32> = HashMap::new();

    let mut chip_handler = None;

    while let Some(give) = gives.pop_front() {
        if give.to.starts_with("output") {
            outputs.insert(give.to.clone(), give.chip);
        } else {
            let bot = bots.get_mut(&give.to).unwrap();
            let gives_from_bot = bot.give_chip(give.chip);
            let given_chips: Vec<u32> = gives_from_bot.iter().map(|g| g.chip).collect();
            if given_chips.contains(&chip1) && given_chips.contains(&chip2) {
                chip_handler = Some(bot.id.clone());
            }

            for give in gives_from_bot {
                gives.push_back(give);
            }
        }
    }

    (chip_handler, outputs)
}

#[aoc(day10, part1)]
fn part1(input: &str) -> String {
    let (chip_handler, _) = find_bot(input, 61, 17);

    chip_handler.unwrap()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u32 {
    let (_, outputs) = find_bot(input, 61, 17);

    outputs.get(&"output 0".to_owned()).unwrap_or(&1)
        * outputs.get(&"output 1".to_owned()).unwrap_or(&1)
        * outputs.get(&"output 2".to_owned()).unwrap_or(&1)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test]
    fn test_find_bot() {
        let (chip_handler, _) = find_bot(TEST_INPUT, 5, 2);
        assert_eq!(chip_handler.unwrap(), "bot 2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 30);
    }
}
