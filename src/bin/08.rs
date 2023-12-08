use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Copy, Clone, Debug)]
enum Inst {
    L,
    R,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Inst::L,
            'R' => Inst::R,
            _ => panic!("not L or R"),
        })
        .collect::<Vec<_>>();

    // Skip blank
    let _ = lines.next();

    let network: HashMap<&str, (&str, &str)> = lines
        .map(|line| {
            let (key, pair_raw) = line.split_once(" = ").unwrap();
            let pair = pair_raw
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.split_once(", "))
                .unwrap();
            (key, pair)
        })
        .collect();

    let mut current = "AAA";
    let mut insts = instructions.iter().cycle().enumerate();

    while current != "ZZZ" {
        current = match insts.next().unwrap().1 {
            Inst::L => network[current].0,
            Inst::R => network[current].1,
        };
    }

    Some(insts.next().unwrap().0 as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_0() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 0,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_0() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 0,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}
