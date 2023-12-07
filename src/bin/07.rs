#![feature(array_windows)]

use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        use Card::*;
        match c {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => panic!("invalid card {c}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    Pair,
    TwoPair,
    Trips,
    Boat,
    Quads,
    Five,
}

impl From<[Card; 5]> for Hand {
    fn from(cards: [Card; 5]) -> Self {
        let mut freq = HashMap::<Card, usize>::new();
        for card in cards {
            *freq.entry(card).or_default() += 1;
        }

        let mut freq_vals = freq.values().copied().collect::<Vec<_>>();
        freq_vals.sort();
        let freq_vals = &*freq_vals;

        if freq_vals.len() == 1 {
            Hand::Five
        } else if freq_vals == [1, 4] {
            Hand::Quads
        } else if freq_vals == [2, 3] {
            Hand::Boat
        } else if freq_vals == [1, 1, 3] {
            Hand::Trips
        } else if freq_vals == [1, 2, 2] {
            Hand::TwoPair
        } else if freq_vals == [1, 1, 1, 2] {
            Hand::Pair
        } else {
            Hand::HighCard
        }
    }
}

#[derive(Debug)]
struct Turn {
    cards: [Card; 5],
    bid: u32,
}

impl Turn {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        Self { cards, bid }
    }
}

impl PartialEq for Turn {
    fn eq(&self, other: &Self) -> bool {
        (0..5).all(|i| self.cards[i] == other.cards[i])
    }
}

impl Eq for Turn {}

impl Ord for Turn {
    fn cmp(&self, other: &Self) -> Ordering {
        let this_hand: Hand = self.cards.into();
        let other_hand: Hand = other.cards.into();

        match this_hand.cmp(&other_hand) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (our_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    let ordering = our_card.cmp(other_card);
                    if matches!(ordering, Ordering::Less | Ordering::Greater) {
                        return ordering;
                    }
                }
                panic!("hands are equal!!!")
            }
        }
    }
}

impl PartialOrd for Turn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut turns = input
        .lines()
        .map(|line| {
            let (cards_raw, bid_raw) = line.split_once(' ').unwrap();
            let cards = cards_raw
                .chars()
                .map(Card::from)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let bid = bid_raw.parse::<u32>().unwrap();
            Turn::new(cards, bid)
        })
        .collect::<Vec<_>>();

    turns.sort();
    let highest_rank = turns.len() as u32;

    Some(
        turns
            .into_iter()
            .zip(1..=highest_rank)
            .map(|(turn, rank)| turn.bid * rank)
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
