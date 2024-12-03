use regex::Regex;

type Input = Vec<(bool, u32, u32)>;
pub fn parse(input: &str) -> Input {
    let mul_regex = Regex::new(r"^(\d+),(\d+)\)").unwrap();
    let mut skipped = false;
    let mut next_skipped = false;

    input
        .split("mul(")
        .filter_map(|split| {
            skipped = next_skipped;
            match (split.find("don't()"), split.find("do()")) {
                (Some(dont_idx), Some(do_idx)) => {
                    next_skipped = dont_idx > do_idx;
                }
                (Some(_), None) => next_skipped = true,
                (None, Some(_)) => next_skipped = false,
                _ => {}
            }

            mul_regex
                .captures(split)
                .map(|nums| (skipped, nums[1].parse().unwrap(), nums[2].parse().unwrap()))
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().map(|(_, a, b)| a * b).sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .filter_map(|(skipped, a, b)| if *skipped { None } else { Some(a * b) })
        .sum()
}
