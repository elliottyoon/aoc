use crate::util::iter::ChunkOps;
use crate::util::parse::ParseOps;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    input
        .iter_unsigned::<u32>()
        .chunk::<2>()
        .map(|[l, r]| (l, r))
        .unzip()
}

pub fn part1(input: &Input) -> u32 {
    let (mut left, mut right) = input.clone();

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part2(_input: &Input) -> u32 {
    42
}
