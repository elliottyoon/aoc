pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &[Vec<i32>]) -> usize {
    input.iter().filter(|row| is_safe(row)).count()
}

pub fn part2(input: &[Vec<i32>]) -> usize {
    input
        .iter()
        .filter(|row| {
            for i in 0..row.len() {
                let new_row = row
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != i)
                    .map(|(_, v)| *v)
                    .collect::<Vec<_>>();
                if is_safe(&new_row) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn is_increasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn is_decreasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] >= w[1])
}

fn is_safe(report: &[i32]) -> bool {
    // always increasing or decreasing
    if !(is_increasing(report) || is_decreasing(report)) {
        return false;
    }

    for i in 0..report.len() - 1 {
        let dist = report[i].abs_diff(report[i + 1]);
        if dist < 1 || dist > 3 {
            return false;
        }
    }
    true

    // AND adjacent levels differ by at least one and at most three.
}
