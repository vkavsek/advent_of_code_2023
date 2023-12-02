use std::collections::BTreeMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    if input.is_empty() {
        return None;
    }
    let res: u32 = input
        .lines()
        .map(|line| {
            let mut res_num = 0;
            let line = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();

            if let Some(first) = line.first() {
                res_num += 10 * first;
            }
            if let Some(last) = line.last() {
                res_num += last;
            }
            res_num
        })
        .sum();

    Some(res)
}
pub fn part_two(input: &str) -> Option<u32> {
    if input.is_empty() {
        return None;
    }
    let res: u32 = input.lines().filter_map(extract_nums).sum();
    Some(res)
}

fn extract_nums(input: &str) -> Option<u32> {
    if input.is_empty() {
        return None;
    }
    let nums_hash = BTreeMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut ordering_map: BTreeMap<usize, u32> = BTreeMap::new();
    for &ch_num in nums_hash.keys().chain(nums_hash.values()) {
        input.match_indices(ch_num).for_each(|(id, _)| {
            ordering_map.insert(
                id,
                nums_hash
                    .get(ch_num)
                    .unwrap_or(&ch_num)
                    .parse()
                    .expect("couldn't parse the number!"),
            );
        });
    }
    let ordered: Vec<_> = ordering_map.values().collect();

    let mut res_val = 0;

    if let Some(&&first) = ordered.first() {
        res_val += 10 * first;
    }
    if let Some(&&last) = ordered.last() {
        res_val += last;
    }

    Some(res_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
