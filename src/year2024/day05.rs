use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Input = (u32, u32);

fn correct_order(pred: &HashMap<u32, HashSet<u32>>, nums: &Vec<u32>) -> HashSet<u32> {
    let mut res = HashSet::new();

    for (idx, num) in nums.iter().enumerate().rev() {
        // check that it's not supposed to be before any elements that are before it
        if let Some(preds) = pred.get(&num) {
            for i in 0..idx {
                if preds.contains(&nums[i]) {
                    // this is an element that should appear later than it currently does
                    res.insert(*num);
                }
            }
        }
    }
    res
}

pub fn parse(input: &str) -> Input {
    let mut part1 = Vec::new();
    let mut part2: Vec<u32> = Vec::new();

    let binding = input.lines().collect::<Vec<_>>();
    let groups = binding
        .splitn(2, |line| line.is_empty())
        .collect::<Vec<_>>();

    // x -> y means x can't come after y
    let mut pred: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in groups[0] {
        let mut segments = line
            .split('|')
            .map(|num| num.parse().unwrap())
            .collect::<Vec<u32>>();
        assert_eq!(segments.len(), 2);
        pred.entry(segments[0])
            .or_insert(HashSet::new())
            .insert(segments[1]);
    }
    for line in groups[1] {
        let mut nums = line
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();
        let bad_set = correct_order(&pred, &nums);
        if bad_set.is_empty() {
            let middle = nums[nums.len() / 2];
            part1.push(middle);
        } else {
            // Correctly order the row and add the middle to part 2
            // aka add the incorrect elements one by one
            let mut good_nums = nums
                .iter()
                .copied()
                .filter(|num| !bad_set.contains(num))
                .collect::<Vec<_>>();
            for num in bad_set {
                // it is in front of everything it needs to be in front of
                for i in 0..good_nums.len() {
                    let mut shit = true;

                    for j in 0..i {
                        // yikes: nums in front of num shouldn't be
                        if let Some(preds) = pred.get(&num) {
                            if preds.contains(&good_nums[j]) {
                                shit = false;
                                break;
                            }
                        }
                    }
                    if !shit {
                        continue;
                    }
                    for j in i..good_nums.len() {
                        if let Some(preds) = pred.get(&good_nums[j]) {
                            if preds.contains(&num) {
                                shit = false;
                                break;
                            }
                        }
                    }
                    if !shit {
                        continue;
                    }
                    good_nums.insert(i, num);
                    break;
                }
            }
            let middle = good_nums[nums.len() / 2];
            part2.push(middle);
        }
    }

    (part1.into_iter().sum(), part2.into_iter().sum())
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
