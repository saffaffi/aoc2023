#![feature(array_windows)]

use std::str::FromStr;

advent_of_code::solution!(9);

fn predict_next(nums: &[i64]) -> i64 {
    if nums.iter().all(|&n| n == 0) {
        0
    } else {
        let diffs = nums
            .array_windows::<2>()
            .map(|[l, r]| r - l)
            .collect::<Vec<_>>();
        let next_diff = predict_next(&diffs);

        nums.last().unwrap() + next_diff
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .flat_map(i64::from_str)
                    .collect::<Vec<_>>()
            })
            .map(|nums| predict_next(&nums))
            .sum(),
    )
}

fn predict_prev(nums: &[i64]) -> i64 {
    if nums.iter().all(|&n| n == 0) {
        0
    } else {
        let diffs = nums
            .array_windows::<2>()
            .map(|[l, r]| r - l)
            .collect::<Vec<_>>();
        let prev_diff = predict_prev(&diffs);

        nums.first().unwrap() - prev_diff
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .flat_map(i64::from_str)
                    .collect::<Vec<_>>()
            })
            .map(|nums| predict_prev(&nums))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
