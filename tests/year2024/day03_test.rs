use aoc::year2024::day03::{parse, part1, part2};

const EXAMPLE_INPUT_1: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const EXAMPLE_INPUT_2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE_INPUT_1);
    assert_eq!(part1(&input), 161);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE_INPUT_2);
    println!("{:?}", EXAMPLE_INPUT_2);
    assert_eq!(part2(&input), 48);
}
