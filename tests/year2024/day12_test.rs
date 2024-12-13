use aoc::year2024::day12::{parse, part1};

const EXAMPLE_INPUT: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 1930);
}
