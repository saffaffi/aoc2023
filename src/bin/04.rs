use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(4);

struct Card {
    winning: HashSet<u32>,
    have: Vec<u32>,
}

impl Card {
    fn score(self) -> u32 {
        self.have
            .into_iter()
            .filter(|have| self.winning.contains(have))
            .fold(None, |acc, _| acc.map(|old| old * 2).or(Some(1)))
            .unwrap_or(0)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (_, numbers) = line.split_once(": ").unwrap();
                let (winning, have) = numbers.split_once(" | ").unwrap();

                let winning = winning
                    .split_whitespace()
                    .flat_map(u32::from_str)
                    .collect::<HashSet<_>>();
                let have = have
                    .split_whitespace()
                    .flat_map(u32::from_str)
                    .collect::<Vec<_>>();

                Card { winning, have }
            })
            .map(Card::score)
            .sum(),
    )
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
        assert_eq!(result, Some(13));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
