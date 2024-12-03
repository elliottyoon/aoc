//! Benchmark macro source: https://github.com/maneatingape/advent-of-code-rust/blob/main/benches/benchmark.rs

#![allow(unstable_features)]
#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($year:tt, $day:tt) => {
        mod $day {
            use aoc::$year::$day::*;
            use std::fs::read_to_string;
            use std::path::Path;
            use std::sync::LazyLock;
            use test::Bencher;

            static DATA: LazyLock<String> = LazyLock::new(|| {
                let year = stringify!($year);
                let day = stringify!($day);
                let path = Path::new("input")
                    .join(year)
                    .join(day)
                    .with_extension("txt");
                read_to_string(path).unwrap()
            });

            #[bench]
            fn parse_bench(b: &mut Bencher) {
                let data = &DATA;
                b.iter(|| parse(data));
            }

            #[bench]
            fn part1_bench(b: &mut Bencher) {
                let input = parse(&DATA);
                b.iter(|| part1(&input));
            }

            #[bench]
            fn part2_bench(b: &mut Bencher) {
                let input = parse(&DATA);
                b.iter(|| part2(&input));
            }
        }
    };
}

mod year2024 {
    benchmark!(year2024, day01);
    benchmark!(year2024, day02);
    benchmark!(year2024, day03);
}
