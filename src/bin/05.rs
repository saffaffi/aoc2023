#![feature(array_chunks)]

use std::{
    ops::Range,
    str::{FromStr, Lines},
};

advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    mappings: Vec<(Range<u64>, i64)>,
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

fn build_maps(lines: &mut Lines) -> Vec<Map> {
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
                src_start..(src_start + len),
                dest_start as i64 - src_start as i64,
            ));
        }
    }

    if let Some(last) = current.take() {
        maps.push(Map { mappings: last });
    }

    maps
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

    let maps = build_maps(lines.by_ref());

    let locations = seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |acc, map| map.translate(acc)));

    locations.min().map(|min| min as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let seeds_raw = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .flat_map(u64::from_str)
        .collect::<Vec<_>>();

    let mut seeds = seeds_raw
        .array_chunks()
        .map(|[start, len]| (*start..(start + len)))
        .collect::<Vec<_>>();
    seeds.sort_by_key(|range| range.start);

    let nonoverlapping = seeds
        .array_chunks()
        .all(|[left, right]| left.end < right.start);
    assert!(nonoverlapping);

    let maps = build_maps(lines.by_ref());

    let mut prev = seeds;
    let mut next = Vec::new();

    for map in maps {
        for (src_range, increment) in map.mappings {
            // There will be sections of ranges (or, even more likely, a bunch
            // of entire ranges!) that weren't included in this mapping. They
            // still need to be considered by future iterations of the mappings
            // loop, so we'll put them into this list and use this as `prev` on
            // the next iteration.
            let mut non_overlapped = Vec::new();

            for old_range in prev {
                if src_range.end <= old_range.start || old_range.end <= src_range.start {
                    // The two ranges don't overlap at all
                    non_overlapped.push(old_range);
                } else if src_range.start < old_range.start && src_range.end < old_range.end {
                    // println!(
                    //     "{:?} inc {} overlaps the start of {:?}",
                    //     src_range, increment, old_range
                    // );

                    // The upper half of the src range overlaps the lower half
                    // of the old range. We need to split the old range and
                    // increment the lower half.
                    {
                        let start = old_range.start as i64 + increment;
                        let end = src_range.end as i64 + increment;
                        next.push(start as u64..end as u64);
                    }

                    // The upper half of the old range remains the same under
                    // this mapping, but needs to be put into `non_overlapped`,
                    // because another mapping might intersect it.
                    {
                        let start = src_range.end + 1;
                        let end = old_range.end;
                        non_overlapped.push(start..end);
                    }
                } else if old_range.start < src_range.start && old_range.end < src_range.end {
                    // println!(
                    //     "{:?} inc {} overlaps the end of {:?}",
                    //     src_range, increment, old_range
                    // );

                    // The lower half of the src range overlaps the upper half
                    // of the old range. We need to split the old range and
                    // increment the upper half.
                    {
                        let start = src_range.start as i64 + increment;
                        let end = old_range.end as i64 + increment;
                        next.push(start as u64..end as u64);
                    }

                    // The lower half of the old range remains the same under
                    // this mapping, but needs to be put into `non_overlapped`,
                    // because another mapping might intersect it.
                    {
                        let start = old_range.start;
                        let end = src_range.start - 1;
                        non_overlapped.push(start..end);
                    }
                } else if src_range.start <= old_range.start && old_range.end <= src_range.end {
                    // println!(
                    //     "{:?} inc {} more than covers {:?}",
                    //     src_range, increment, old_range
                    // );

                    // Increment the whole old range.
                    {
                        let start = old_range.start as i64 + increment;
                        let end = old_range.end as i64 + increment;
                        next.push(start as u64..end as u64);
                    }
                } else if old_range.start <= src_range.start && src_range.end <= old_range.end {
                    // println!(
                    //     "{:?} inc {} is inside {:?}",
                    //     src_range, increment, old_range
                    // );

                    // The entire src range overlaps a section in the middle of
                    // the old range. We need to split the old range and
                    // increment the middle section. The middle section *is* the
                    // src range.
                    {
                        let start = src_range.start as i64 + increment;
                        let end = src_range.end as i64 + increment;
                        next.push(start as u64..end as u64);
                    }

                    // The lower half of the old range remains the same under
                    // this mapping, but needs to be put into `non_overlapped`,
                    // because another mapping might intersect it.
                    {
                        let start = old_range.start;
                        let end = src_range.start;
                        non_overlapped.push(start..end);
                    }

                    // Same for the upper half.
                    {
                        let start = src_range.end;
                        let end = old_range.end;
                        non_overlapped.push(start..end);
                    }
                } else {
                    panic!("unconsidered case: {:?} and {:?}", old_range, src_range);
                }
            }

            prev = non_overlapped;
        }

        // Anything not overlapped by *any* mapping needs to be put into
        // `next` as well.
        next.append(&mut prev);

        prev = next;
        next = Vec::new();
    }

    prev.into_iter()
        .min_by_key(|range| range.start)
        .map(|range| range.start as u32)
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
