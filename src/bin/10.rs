use core::fmt;
use std::{collections::HashSet, iter, mem};

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

fn grid_and_loop(input: &str) -> (Vec<Vec<Tile>>, Vec<(usize, usize)>) {
    let mut start = None;
    let mut grid = input
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
    let mut nexts: [(usize, usize); 2] = adjacent_to(start, lens)
        .filter(|&(next_x, next_y)| {
            grid[next_y][next_x]
                .connections((next_x, next_y), lens)
                .any(|conn| conn == start)
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    nexts.sort();

    let new_start = 'calc_start: {
        for tile in [
            Tile::Pipe,
            Tile::Dash,
            Tile::L,
            Tile::J,
            Tile::Seven,
            Tile::F,
        ] {
            let mut mb_conns = tile.connections(start, lens).collect::<Vec<_>>();
            mb_conns.sort();

            if mb_conns == nexts {
                break 'calc_start tile;
            }
        }

        panic!("no new start found");
    };

    grid[start.1][start.0] = new_start;

    let mut prev = start;
    let mut curr = nexts[0];

    let mut loop_ = vec![start, curr];

    while curr != start {
        let next = grid[curr.1][curr.0]
            .connections(curr, lens)
            .find(|&next| next != prev)
            .unwrap();
        loop_.push(next);
        prev = curr;
        curr = next;
    }

    (grid, loop_)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_grid, loop_) = grid_and_loop(input);
    Some(loop_.len() as u32 / 2)
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pipe => write!(f, "|"),
            Self::Dash => write!(f, "-"),
            Self::L => write!(f, "L"),
            Self::J => write!(f, "J"),
            Self::Seven => write!(f, "7"),
            Self::F => write!(f, "F"),
            Self::Ground => write!(f, "."),
            Self::Start => write!(f, "S"),
        }
    }
}

#[allow(dead_code)]
fn print_grid<T>(grid: &Vec<Vec<T>>)
where
    T: fmt::Display,
{
    for row in grid {
        for tile in row {
            print!("{}", tile);
        }
        println!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum SearchTile {
    Pipe,
    Dash,
    L,
    J,
    Seven,
    F,
    Ground,
    Start,
    SolidH,
    SolidV,
    Open,
    Out,
}

impl From<Tile> for SearchTile {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Pipe => Self::Pipe,
            Tile::Dash => Self::Dash,
            Tile::L => Self::L,
            Tile::J => Self::J,
            Tile::Seven => Self::Seven,
            Tile::F => Self::F,
            Tile::Ground => Self::Ground,
            Tile::Start => Self::Start,
        }
    }
}

impl fmt::Display for SearchTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pipe => write!(f, "|"),
            Self::Dash => write!(f, "-"),
            Self::L => write!(f, "L"),
            Self::J => write!(f, "J"),
            Self::Seven => write!(f, "7"),
            Self::F => write!(f, "F"),
            Self::Ground => write!(f, "."),
            Self::Start => write!(f, "S"),
            Self::SolidH => write!(f, "~"),
            Self::SolidV => write!(f, "'"),
            Self::Open => write!(f, " "),
            Self::Out => write!(f, "O"),
        }
    }
}

fn explode_grid(orig: Vec<Vec<Tile>>) -> Vec<Vec<SearchTile>> {
    let rows = orig.len();
    let cols = orig[0].len();

    let mut new = Vec::with_capacity(rows * 2);

    for (orig_y, row) in orig.iter().cloned().enumerate() {
        let mut new_row = Vec::with_capacity(cols * 2);
        let mut new_extra_row = Vec::with_capacity(cols * 2);

        for (orig_x, tile) in row.iter().copied().enumerate() {
            let conns = tile
                .connections((orig_x, orig_y), (cols, rows))
                .collect::<Vec<_>>();

            new_row.push(tile.into());

            let contains_right = conns.contains(&(orig_x + 1, orig_y));
            let right_contains = if let Some(right_tile) = row.get(orig_x + 1) {
                let right_conns = right_tile
                    .connections((orig_x + 1, orig_y), (cols, rows))
                    .collect::<Vec<_>>();
                right_conns.contains(&(orig_x, orig_y))
            } else {
                false
            };

            if contains_right && right_contains {
                new_row.push(SearchTile::SolidH);
            } else {
                new_row.push(SearchTile::Open);
            }

            let contains_down = conns.contains(&(orig_x, orig_y + 1));
            let down_contains = if let Some(down_tile) = orig.get(orig_y + 1).map(|r| r[orig_x]) {
                let down_conns = down_tile
                    .connections((orig_x, orig_y + 1), (cols, rows))
                    .collect::<Vec<_>>();
                down_conns.contains(&(orig_x, orig_y))
            } else {
                false
            };

            if contains_down && down_contains {
                new_extra_row.push(SearchTile::SolidV);
            } else {
                new_extra_row.push(SearchTile::Open);
            }

            new_extra_row.push(SearchTile::Open);
        }

        new.push(new_row);
        new.push(new_extra_row);
    }

    new
}

#[allow(dead_code)]
fn print_loop_out(grid: &[Vec<SearchTile>], in_loop: &impl Fn((usize, usize)) -> bool) {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile == &SearchTile::Out || in_loop((x, y)) {
                print!("{}", tile);
            } else {
                print!(" ");
            }
        }
        println!()
    }
}

fn paint_grid(
    grid: &[Vec<SearchTile>],
    in_loop: impl Fn((usize, usize)) -> bool,
) -> Vec<Vec<SearchTile>> {
    let mut grid = grid.to_owned();

    let rows = grid.len();
    let cols = grid[0].len();

    let top_row = (0..cols).zip(iter::repeat(0));
    let bottom_row = (0..cols).zip(iter::repeat(rows - 1));
    let left_col = iter::repeat(0).zip(1..rows - 1);
    let right_col = iter::repeat(cols - 1).zip(1..rows - 1);

    let outside = top_row
        .chain(bottom_row)
        .chain(left_col)
        .chain(right_col)
        .collect::<Vec<_>>();
    let mut changed = Vec::with_capacity(outside.len());

    // print_loop_out(&grid, &in_loop);

    for (x, y) in outside {
        if !in_loop((x, y)) {
            grid[y][x] = SearchTile::Out;
            changed.push((x, y));
        }
    }

    // print_loop_out(&grid, &in_loop);
    // println!();

    while !changed.is_empty() {
        let last_changed = mem::take(&mut changed);
        for pos in last_changed {
            for (n_x, n_y) in adjacent_to(pos, (cols, rows)) {
                // if matches!(grid[n_y][n_x], SearchTile::Ground | SearchTile::Open) {
                //     grid[n_y][n_x] = SearchTile::Out;
                //     changed.push((n_x, n_y));
                // }
                if !(matches!(grid[n_y][n_x], SearchTile::Out) || in_loop((n_x, n_y))) {
                    grid[n_y][n_x] = SearchTile::Out;
                    changed.push((n_x, n_y));
                }
            }
        }

        // print_loop_out(&grid, &in_loop);
        // println!();
    }

    grid
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, loop_) = grid_and_loop(input);

    let loop_coords = loop_.into_iter().collect::<HashSet<_>>();
    let in_orig_loop = move |(x, y)| loop_coords.contains(&(x, y));

    // print_grid(&grid);
    // println!();

    let exploded = explode_grid(grid);

    // print_grid(&exploded);
    // println!();

    let in_exploded_loop = |(x, y): (usize, usize)| {
        if x % 2 == 0 && y % 2 == 0 {
            in_orig_loop((x / 2, y / 2))
        // } else if x % 2 != 0 && y % 2 != 0 {
        //     false
        } else if exploded[y][x] == SearchTile::SolidV {
            let orig_x = ((x as f32) / 2.).floor() as usize;
            let orig_y_low = ((y as f32) / 2.).floor() as usize;
            let orig_y_high = ((y as f32) / 2.).ceil() as usize;

            in_orig_loop((orig_x, orig_y_low)) && in_orig_loop((orig_x, orig_y_high))
        } else if exploded[y][x] == SearchTile::SolidH {
            let orig_x_low = ((x as f32) / 2.).floor() as usize;
            let orig_x_high = ((x as f32) / 2.).ceil() as usize;
            let orig_y = ((y as f32) / 2.).ceil() as usize;

            in_orig_loop((orig_x_low, orig_y)) && in_orig_loop((orig_x_high, orig_y))
        } else {
            false
        }
    };

    let painted = paint_grid(&exploded, in_exploded_loop);
    let inside = painted
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter().enumerate().filter(move |&(x, tile)| {
                !matches!(
                    tile,
                    SearchTile::Out | SearchTile::SolidV | SearchTile::SolidH | SearchTile::Open
                ) && !in_exploded_loop((x, y))
            })
        })
        .count();
    Some(inside)
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
    fn test_part_two_simple() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_larger() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_random_tiles() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(10));
    }
}
