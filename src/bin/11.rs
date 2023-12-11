use std::{collections::HashSet, mem};

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn expand(input: &str) -> Vec<Vec<char>> {
    let mut grid = parse(input);

    let no_star_cols: HashSet<usize> = grid
        .iter()
        .flat_map(|row| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(|(i, _)| i)
        })
        .fold(HashSet::from_iter(0..grid[0].len()), |mut acc, i| {
            acc.remove(&i);
            acc
        });
    let mut no_star_cols = no_star_cols.into_iter().collect::<Vec<_>>();
    no_star_cols.sort();

    for mut row in mem::take(&mut grid) {
        let starless = !row.contains(&'#');

        for &col in no_star_cols.iter().rev() {
            row.insert(col + 1, '.');
        }

        if starless {
            grid.push(row.clone());
        }

        grid.push(row);
    }

    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = expand(input);
    let stars = grid
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
        })
        .collect::<Vec<_>>();

    let mut total = 0;

    for (x1, y1) in stars.iter().copied() {
        for (x2, y2) in stars.iter().copied() {
            if x1 == x2 && y1 == y2 {
                continue;
            }

            total += x1.max(x2) - x1.min(x2);
            total += y1.max(y2) - y1.min(y2);
        }
    }

    Some((total / 2) as u32)
}

fn part_two_inner(input: &str, factor: usize) -> u64 {
    let grid = parse(input);

    let no_star_cols: HashSet<usize> = grid
        .iter()
        .flat_map(|row| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(|(i, _)| i)
        })
        .fold(HashSet::from_iter(0..grid[0].len()), |mut acc, i| {
            acc.remove(&i);
            acc
        });
    let mut no_star_cols = no_star_cols.into_iter().collect::<Vec<_>>();
    no_star_cols.sort();

    let no_star_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| (!row.contains(&'#')).then_some(i))
        .collect();

    let mut stars = grid
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some(((x, 0), (y, 0))))
        })
        .collect::<Vec<_>>();

    for ((x, add_x), (y, add_y)) in &mut stars {
        for i in &no_star_rows {
            if i < y {
                *add_y += factor - 1;
            }
        }

        for i in &no_star_cols {
            if i < x {
                *add_x += factor - 1;
            }
        }
    }

    let stars: Vec<_> = stars
        .into_iter()
        .map(|((x, add_x), (y, add_y))| (x + add_x, y + add_y))
        .collect();

    let mut total = 0;

    for (x1, y1) in stars.iter().copied() {
        for (x2, y2) in stars.iter().copied() {
            if x1 == x2 && y1 == y2 {
                continue;
            }

            total += x1.max(x2) - x1.min(x2);
            total += y1.max(y2) - y1.min(y2);
        }
    }

    (total / 2) as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(part_two_inner(input, 1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn part_one_expand() {
        let expanded = expand(&advent_of_code::template::read_file("examples", DAY));
        let parsed = parse(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(expanded, parsed);
    }

    #[test]
    fn test_part_two_10() {
        let result = part_two_inner(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 1030);
    }
}
