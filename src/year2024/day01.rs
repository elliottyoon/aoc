use std::collections::BinaryHeap;

type Input = Vec<(u32, u32)>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next()?.parse::<u32>().ok()?;
            let second = parts.next()?.parse::<u32>().ok()?;
            Some((first, second))
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    // split input into two sorted vectors, and then just iterate through them
    let mut first = BinaryHeap::with_capacity(input.len());
    let mut second = BinaryHeap::with_capacity(input.len());

    for (f, s) in input.into_iter() {
        first.push(f);
        second.push(s);
    }
    debug_assert_eq!(first.len(), second.len());
    let mut total = 0;
    for i in 0..first.len() {
        if let Some(f) = first.pop() {
            if let Some(s) = second.pop() {
                total += u32::abs_diff(*f, *s);
            }
        }
    }
    total
}

pub fn part2(_input: &Input) -> u32 {
    42
}
