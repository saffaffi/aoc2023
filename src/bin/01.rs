advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                let mut digits = line
                    .matches(char::is_numeric)
                    .flat_map(|str| str.parse::<u32>());
                let first = digits.next();
                first.zip(digits.last().or(first))
            })
            .map(|(first, last)| (first * 10) + last)
            .sum(),
    )
}

#[rustfmt::skip]
mod make {
    pub fn one() -> u32 { 1 }
    pub fn two() -> u32 { 2 }
    pub fn three() -> u32 { 3 }
    pub fn four() -> u32 { 4 }
    pub fn five() -> u32 { 5 }
    pub fn six() -> u32 { 6 }
    pub fn seven() -> u32 { 7 }
    pub fn eight() -> u32 { 8 }
    pub fn nine() -> u32 { 9 }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                let each_str_matched = [
                    ("one", make::one as fn() -> u32),
                    ("two", make::two as fn() -> u32),
                    ("three", make::three as fn() -> u32),
                    ("four", make::four as fn() -> u32),
                    ("five", make::five as fn() -> u32),
                    ("six", make::six as fn() -> u32),
                    ("seven", make::seven as fn() -> u32),
                    ("eight", make::eight as fn() -> u32),
                    ("nine", make::nine as fn() -> u32),
                ]
                .map(|(needle, func)| line.match_indices(needle).map(move |(i, _)| (i, func())));

                let digits = line
                    .match_indices(char::is_numeric)
                    .map(|(i, val)| (i, val.parse::<u32>().unwrap()));

                let mut all = each_str_matched.into_iter().fold(
                    digits.collect::<Vec<_>>(),
                    |mut all, next| {
                        all.append(&mut next.collect());
                        all
                    },
                );

                all.sort_by_key(|&(i, _)| i);

                let first = all.first().map(|it| it.1);
                first.zip(all.last().map(|it| it.1).or(first))
            })
            .map(|(first, last)| (first * 10) + last)
            .sum(),
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
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
