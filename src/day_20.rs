use std::ops::RangeInclusive;

use aoc_runner_derive::aoc;

type IpRange = RangeInclusive<u32>;

fn parse_blacklist(input: &str) -> Vec<IpRange> {
    input
        .lines()
        .map(|l| {
            let (start_str, end_str) = l.split_once('-').unwrap();
            let start = start_str.parse::<u32>().unwrap();
            let end = end_str.parse::<u32>().unwrap();
            start..=end
        })
        .collect()
}

fn split(range: &IpRange, block: &IpRange) -> Vec<IpRange> {
    let (range_s, range_e) = (*range.start(), *range.end());
    let (block_s, block_e) = (*block.start(), *block.end());

    // no overlap
    if block_s > range_e || block_e < range_s {
        return vec![range.clone()];
    }

    // covered
    if block_s <= range_s && block_e >= range_e {
        return vec![];
    }

    // start blocked
    if block_s <= range_s && block_e < range_e {
        return vec![block_e + 1..=range_e];
    }

    // end blocked
    if block_s <= range_e && range_e <= block_e {
        return vec![range_s..=block_s - 1];
    }

    // middle blocked
    if block_s > range_s && block_e < range_e {
        return vec![range_s..=block_s - 1, block_e + 1..=range_e];
    }

    panic!(
        "Unexpected block coverage! range: {:?} block: {:?}",
        range, block
    );
}

fn valid_ranges(input: &str, total_range: IpRange) -> Vec<IpRange> {
    let blacklist = parse_blacklist(input);

    let mut valid_ranges = vec![total_range];

    for block in blacklist {
        valid_ranges = valid_ranges
            .iter()
            .flat_map(|range| split(range, &block))
            .collect()
    }

    valid_ranges
}

fn solver_part1(input: &str, total_range: IpRange) -> u32 {
    let mut free_ip_ranges = valid_ranges(input, total_range);

    free_ip_ranges.sort_by_key(|r| *r.start());

    *free_ip_ranges.get(0).expect("No valid IPs").start()
}

fn solver_part2(input: &str, total_range: IpRange) -> u32 {
    let free_ip_ranges = valid_ranges(input, total_range);

    free_ip_ranges
        .iter()
        .map(|r| r.end() - r.start() + 1) // +1 for inclusivity
        .sum()
}

#[aoc(day20, part1)]
fn part1(input: &str) -> u32 {
    solver_part1(input, 0..=4294967295_u32)
}

#[aoc(day20, part2)]
fn part2(input: &str) -> u32 {
    solver_part2(input, 0..=4294967295_u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        // no overlap
        assert_eq!(split(&(0..=2_u32), &(10..=11_u32)), vec![0..=2_u32]);
        assert_eq!(split(&(100..=102_u32), &(10..=11_u32)), vec![100..=102_u32]);

        // completely blocked
        assert_eq!(split(&(1..=2_u32), &(0..=3_u32)), vec![]);

        // start blocked
        assert_eq!(split(&(3..=8_u32), &(0..=5_u32)), vec![6..=8_u32]);

        // end blocked
        assert_eq!(split(&(0..=10_u32), &(6..=15_u32)), vec![0..=5_u32]);
        assert_eq!(
            split(
                &(2447228621_u32..=2451978745_u32),
                &(2447228622_u32..=2451978745_u32),
            ),
            vec![2447228621_u32..=2447228621_u32]
        );

        // middle blocked
        assert_eq!(
            split(&(0..=20_u32), &(8..=11_u32)),
            vec![0..=7_u32, 12..=20_u32]
        );
    }

    static TEST_INPUT: &str = "5-8\n0-2\n4-7";

    #[test]
    fn test_part1() {
        assert_eq!(solver_part1(TEST_INPUT, 0..=9_u32), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solver_part2(TEST_INPUT, 0..=9_u32), 2);
    }
}
