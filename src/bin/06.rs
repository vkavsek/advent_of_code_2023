use indicatif::ProgressIterator;
use itertools::Itertools;

advent_of_code::solution!(6);

fn parse_to_u64(input: &str) -> u64 {
    input.parse().expect("number should be parsable")
}
pub fn part_one(input: &str) -> Option<u64> {
    let (times, records) = input.split_once('\n').expect("not valid input");
    let op = times
        .trim_start_matches("Time:")
        .split_whitespace()
        .zip(records.trim_start_matches("Distance:").split_whitespace())
        .map(|(time, record)| (parse_to_u64(time), parse_to_u64(record)))
        .collect_vec();

    let mut res = 1;
    op.iter().for_each(|(time, record)| {
        let mut acc = 0;
        for time_held in 1..*time {
            if time_held * (*time - time_held) > *record {
                acc += 1;
            }
        }
        res *= acc;
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (times, records) = input.split_once('\n').expect("not valid input");
    let time = parse_to_u64(
        &times
            .trim_start_matches("Time:")
            .split_whitespace()
            .flat_map(|t| t.chars())
            .collect::<String>(),
    );

    let record = parse_to_u64(
        &records
            .trim_start_matches("Distance:")
            .split_whitespace()
            .flat_map(|t| t.chars())
            .collect::<String>(),
    );

    let mut acc = 0;
    for time_held in (0..time).progress_count(time / 4) {
        let time_left = time - time_held;
        if time_held * time_left < record {
            acc += 1;
        } else {
            break;
        }
    }

    Some(time + 1 - 2 * acc)
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
