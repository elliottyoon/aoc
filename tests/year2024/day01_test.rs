use aoc::year2024::day01::{parse, part1};

const EXAMPLE_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

const EXAMPLE_INPUT_2: &str = "
";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 11);
}

#[test]
fn part2_test() {
    todo!();
}
