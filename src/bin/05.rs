use std::{ops::RangeInclusive, str::FromStr};

advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    mappings: Vec<(RangeInclusive<u64>, i64)>,
}

impl Map {
    fn translate(&self, source: u64) -> u64 {
        for (range, increment) in &self.mappings {
            if range.contains(&source) {
                return (source as i64 + increment) as u64;
            }
        }

        source
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .flat_map(u64::from_str)
        .collect::<Vec<_>>();

    let mut maps = Vec::new();
    let mut current = None;

    while let Some(line) = lines.next() {
        if line.is_empty() {
            // Beginning of a new block
            let line = lines.next().unwrap();
            assert!(line.ends_with(" map:"));

            if let Some(last) = current.take() {
                maps.push(Map { mappings: last });
            }

            current = Some(Vec::new());
        } else {
            let [dest_start, src_start, len] = line
                .split_whitespace()
                .flat_map(u64::from_str)
                .take(3)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let cur = current.as_mut().unwrap();

            cur.push((
                src_start..=(src_start + len),
                dest_start as i64 - src_start as i64,
            ));
        }
    }

    if let Some(last) = current.take() {
        maps.push(Map { mappings: last });
    }

    let locations = seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |acc, map| map.translate(acc)));

    locations.min().map(|min| min as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
