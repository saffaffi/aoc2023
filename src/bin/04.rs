use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(4);

struct Card {
    winning: HashSet<u32>,
    have: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        self.have
            .iter()
            .filter(|have| self.winning.contains(have))
            .fold(None, |acc, _| acc.map(|old| old * 2).or(Some(1)))
            .unwrap_or(0)
    }
}

fn parse_cards(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines().map(|line| {
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
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_cards(input).map(|c| c.score()).sum())
}

impl Card {
    fn matching(&self) -> usize {
        self.have
            .iter()
            .filter(|&have| self.winning.contains(have))
            .count()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_cards = input.lines().count();
    let mut copies = vec![1; num_cards];

    for (i, card) in parse_cards(input).enumerate() {
        let matching = card.matching();
        let i_copies = copies[i];

        for c in &mut copies[(i + 1)..((i + 1 + matching).min(num_cards))] {
            *c += i_copies;
        }
    }

    Some(copies.into_iter().sum())
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
