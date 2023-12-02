use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(2);

struct Game {
    id: usize,
    max_seen: HashMap<Colour, usize>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl FromStr for Colour {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

fn parse_game(line: &str) -> Game {
    let (game_in, pulls_in) = line.split_once(": ").unwrap();
    let id = game_in[5..].parse::<usize>().unwrap();

    let mut game = Game {
        id,
        max_seen: Default::default(),
    };

    for pull_in in pulls_in.split("; ") {
        for cubes_in in pull_in.split(", ") {
            let (num, col) = cubes_in.split_once(' ').unwrap();
            let num = num.parse::<usize>().unwrap();
            let col = col.parse::<Colour>().unwrap();

            let entry = game.max_seen.entry(col).or_default();
            *entry = (*entry).max(num);
        }
    }

    game
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(parse_game).collect::<Vec<_>>();

    let mut total = 0;

    for game in games {
        if game.max_seen[&Colour::Red] <= 12
            && game.max_seen[&Colour::Green] <= 13
            && game.max_seen[&Colour::Blue] <= 14
        {
            total += game.id;
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines().map(parse_game).collect::<Vec<_>>();

    Some(
        games
            .into_iter()
            .map(|game| {
                game.max_seen[&Colour::Red]
                    * game.max_seen[&Colour::Green]
                    * game.max_seen[&Colour::Blue]
            })
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
