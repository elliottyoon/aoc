use aoc::year2024::day04::{parse, part1, part2};
const EXAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part1(&input), 18);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part2(&input), 9);
}
