use regex::Regex;

type Input = Vec<(bool, i32, i32)>;
pub fn parse(input: &str) -> Input {
    let mut skipped = false;
    let mut next_skipped = false;

    let mult_regex = Regex::new(r"^(\d+),(\d+)\)").unwrap();

    input
        .split("mul(")
        .filter_map(|thing| {
            skipped = next_skipped;

            let ret = mult_regex
                .captures(thing)
                .map(|nums| (nums[1].parse().unwrap(), nums[2].parse().unwrap()));

            let dont_idx = thing.find("don't()");
            let do_idx = thing.find("do()");
            match (dont_idx, do_idx) {
                (Some(dont), Some(do_)) => {
                    next_skipped = dont > do_;
                }
                (Some(_), None) => next_skipped = true,
                (None, Some(_)) => next_skipped = false,
                _ => {}
            }
            ret.map(|(a, b)| (skipped, a, b))
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &Input) -> i32 {
    input.iter().map(|(_skipped, a, b)| a * b).sum()
}

pub fn part2(input: &Input) -> i32 {
    input
        .iter()
        .filter_map(|(skipped, a, b)| if *skipped { None } else { Some(a * b) })
        .sum()
}
