use aoc::year2024::day05::{parse, part1, part2};

const EXAMPLE_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 143);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 123);
}
