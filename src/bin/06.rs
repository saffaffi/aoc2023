advent_of_code::solution!(6);

use std::str::FromStr;

fn races(input: &str) -> Vec<(u32, u32)> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(u32::from_str);
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(u32::from_str);
    times.zip(distances).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = races(input);

    Some(
        races
            .into_iter()
            .map(|(total_time, record_distance)| {
                (0..=total_time)
                    .filter(move |button_time| {
                        let speed = button_time;
                        let moving_time = total_time - button_time;
                        let distance = speed * moving_time;
                        distance > record_distance
                    })
                    .count() as u32
            })
            .product(),
    )
}

fn race(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    (time, distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (total_time, record_distance) = race(input);

    Some(
        (0..=total_time)
            .filter(move |button_time| {
                let speed = button_time;
                let moving_time = total_time - button_time;
                let distance = speed * moving_time;
                distance > record_distance
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
