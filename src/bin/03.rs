use std::{collections::HashMap, iter, ops::RangeInclusive};

advent_of_code::solution!(3);

struct Number {
    value: u32,
    row: usize,
    cols: RangeInclusive<usize>,
}

impl Number {
    fn new(first_digit: u32, row: usize, col: usize) -> Self {
        Self {
            value: first_digit,
            row,
            cols: col..=col,
        }
    }

    fn extend(&mut self, next_digit: u32) {
        self.value = (self.value * 10) + next_digit;
        self.cols = (*self.cols.start())..=(self.cols.end() + 1);
    }
}

struct SymbolMap {
    symbol_fields: HashMap<(usize, usize), Option<usize>>,
    gears: Vec<(Option<u32>, Option<u32>)>,
    max_row: usize,
    max_col: usize,
}

impl SymbolMap {
    fn with_bounds(max_row: usize, max_col: usize) -> Self {
        Self {
            symbol_fields: HashMap::default(),
            gears: Vec::default(),
            max_row,
            max_col,
        }
    }

    fn add_symbol(&mut self, row: usize, col: usize, is_gear: bool) {
        let row_start = row.saturating_sub(1);
        let row_end = (row + 1).min(self.max_row);

        let col_start = col.saturating_sub(1);
        let col_end = (col + 1).min(self.max_col);

        let gear_idx = if is_gear {
            let idx = self.gears.len();
            self.gears.push((None, None));
            Some(idx)
        } else {
            None
        };

        for row in row_start..=row_end {
            for col in col_start..=col_end {
                self.symbol_fields.insert((row, col), gear_idx);
            }
        }
    }

    fn touches(&mut self, number: &Number) -> bool {
        iter::repeat(number.row)
            .zip(number.cols.clone())
            .any(|(row, col)| match self.symbol_fields.get(&(row, col)) {
                Some(&Some(gear_idx)) => {
                    let gear = self.gears.get_mut(gear_idx).unwrap();

                    match gear {
                        (first @ None, None) => {
                            *first = Some(number.value);
                        }
                        (Some(_), second @ None) => {
                            *second = Some(number.value);
                        }
                        both @ (Some(_), Some(_)) => {
                            *both = (None, None);
                        }
                        (None, Some(_)) => unreachable!(),
                    }

                    true
                }
                Some(None) => true,
                None => false,
            })
    }
}

fn read_schematic(input: &str) -> (Vec<Number>, SymbolMap) {
    let mut lines = input.lines().peekable();
    let cols = lines.peek().unwrap().len();
    let rows = lines.count();

    let mut numbers = Vec::<Number>::new();
    let mut current_number = None::<Number>;

    let mut symbol_map = SymbolMap::with_bounds(rows, cols);

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    if let Some(number) = current_number.take() {
                        numbers.push(number);
                    }
                }
                num_char if num_char.is_numeric() => {
                    let digit = num_char.to_digit(10).unwrap();

                    if let Some(number) = current_number.as_mut() {
                        number.extend(digit);
                    } else {
                        current_number = Some(Number::new(digit, row, col));
                    }
                }
                sym_char => {
                    let is_gear = sym_char == '*';
                    symbol_map.add_symbol(row, col, is_gear);

                    if let Some(number) = current_number.take() {
                        numbers.push(number);
                    }
                }
            }
        }
    }

    if let Some(number) = current_number.take() {
        numbers.push(number);
    }

    (numbers, symbol_map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, mut symbol_map) = read_schematic(input);

    Some(
        numbers
            .into_iter()
            .filter(|num| symbol_map.touches(num))
            .map(|num| num.value)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (numbers, mut symbol_map) = read_schematic(input);

    for num in numbers {
        let _ = symbol_map.touches(&num);
    }

    Some(
        symbol_map
            .gears
            .into_iter()
            .filter_map(|(fst, snd)| fst.zip(snd))
            .map(|(fst, snd)| fst * snd)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
