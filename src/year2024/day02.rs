use crate::util::parse::ParseOps;

type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut row = Vec::new();
    input.lines().for_each(|line| {
        row.extend(line.iter_unsigned::<u32>());
        if is_safe(&row) {
            part1 += 1;
            part2 += 1;
        } else {
            for i in 0..row.len() {
                let level = row.remove(i);
                if is_safe(&row) {
                    part2 += 1;
                    break;
                }
                row.insert(i, level);
            }
        }
        row.clear()
    });

    (part1, part2)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

fn is_safe(report: &[u32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        increasing &= window[0] < window[1];
        decreasing &= window[0] > window[1];
        // Each pair of adjacent levels must be x distance from each other,
        // where 1 <= x <= 3.
        if !(1..=3).contains(&window[0].abs_diff(window[1])) {
            return false;
        }
    }

    increasing ^ decreasing
}
