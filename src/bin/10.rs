advent_of_code::solution!(10);

fn some_if<T, F>(t: T, predicate: F) -> Option<T>
where
    F: for<'a> Fn(&'a T) -> bool,
{
    if predicate(&t) {
        Some(t)
    } else {
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Pipe,
    Dash,
    L,
    J,
    Seven,
    F,
    Ground,
    Start,
}

impl Tile {
    fn connections(
        &self,
        (at_x, at_y): (usize, usize),
        (x_len, y_len): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        match self {
            Tile::Pipe => vec![
                (Some(at_x), at_y.checked_sub(1)),
                (Some(at_x), some_if(at_y + 1, |&new| new < y_len)),
            ],
            Tile::Dash => vec![
                (at_x.checked_sub(1), Some(at_y)),
                (some_if(at_x + 1, |&new| new < x_len), Some(at_y)),
            ],
            Tile::L => vec![
                (some_if(at_x + 1, |&new| new < x_len), Some(at_y)),
                (Some(at_x), at_y.checked_sub(1)),
            ],
            Tile::J => vec![
                (at_x.checked_sub(1), Some(at_y)),
                (Some(at_x), at_y.checked_sub(1)),
            ],
            Tile::Seven => vec![
                (at_x.checked_sub(1), Some(at_y)),
                (Some(at_x), some_if(at_y + 1, |&new| new < y_len)),
            ],
            Tile::F => vec![
                (some_if(at_x + 1, |&new| new < x_len), Some(at_y)),
                (Some(at_x), some_if(at_y + 1, |&new| new < y_len)),
            ],
            Tile::Ground => vec![],
            Tile::Start => vec![],
        }
        .into_iter()
        .filter_map(|(x, y)| x.zip(y))
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::Pipe,
            '-' => Self::Dash,
            'L' => Self::L,
            'J' => Self::J,
            '7' => Self::Seven,
            'F' => Self::F,
            '.' => Self::Ground,
            'S' => Self::Start,
            other => panic!("non-tile character '{other}'"),
        }
    }
}

fn adjacent_to(
    (x, y): (usize, usize),
    (x_len, y_len): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    vec![
        (some_if(x + 1, |&new| new < x_len), Some(y)),
        (
            some_if(x + 1, |&new| new < x_len),
            some_if(y + 1, |&new| new < y_len),
        ),
        (Some(x), some_if(y + 1, |&new| new < y_len)),
        (x.checked_sub(1), some_if(y + 1, |&new| new < y_len)),
        (x.checked_sub(1), Some(y)),
        (x.checked_sub(1), y.checked_sub(1)),
        (Some(x), y.checked_sub(1)),
        (some_if(x + 1, |&new| new < x_len), y.checked_sub(1)),
    ]
    .into_iter()
    .filter_map(|(x, y)| x.zip(y))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = Tile::from(c);
                    if tile == Tile::Start {
                        start = Some((x, y));
                    }
                    tile
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start = start.unwrap();

    let lens = (grid[0].len(), grid.len());

    // Find the two tiles around the start that connect to the start.
    let nexts: [(usize, usize); 2] = adjacent_to(start, lens)
        .filter(|&(next_x, next_y)| {
            grid[next_y][next_x]
                .connections((next_x, next_y), lens)
                .any(|conn| conn == start)
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut prev = start;
    let mut curr = nexts[0];

    let mut loop_ = vec![start];

    while curr != start {
        let next = grid[curr.1][curr.0]
            .connections(curr, lens)
            .find(|&next| next != prev)
            .unwrap();
        loop_.push(next);
        prev = curr;
        curr = next;
    }

    Some(loop_.len() as u32 / 2)
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
