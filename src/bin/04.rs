use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|line| {
            let (_card, line_data) = line.split_once(':').expect("no ':' in a line");
            let (winning_ns, user_ns) = line_data.trim().split_once('|').expect("no '|' in a line");

            let mut points_pow = 0u32;
            for win_n in winning_ns.split_ascii_whitespace() {
                let longer = format!(" {} ", win_n);

                if win_n.len() > 1 && user_ns.contains(win_n)
                    || win_n.len() == 1 && [" ", user_ns, " "].concat().contains(&longer)
                {
                    points_pow += 1;
                }
            }
            if points_pow == 0 {
                points_pow
            } else {
                2u32.pow(points_pow - 1)
            }
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = input
        .lines()
        .map(|line| {
            let (_card, line_data) = line.split_once(':').expect("no ':' in a line");
            let (winning_ns, user_ns) = line_data.trim().split_once('|').expect("no '|' in a line");

            let mut card_num = 0u32;

            for win_n in winning_ns.split_whitespace() {
                let longer = format!(" {} ", win_n);

                if win_n.len() > 1 && user_ns.contains(win_n)
                    || win_n.len() == 1 && [" ", user_ns, " "].concat().contains(&longer)
                {
                    card_num += 1;
                }
            }
            card_num
        })
        .collect_vec();

    let res = {
        let mut counts = vec![1; data.len()];
        for (idx, &new_cards) in data.iter().enumerate().rev() {
            counts[idx] += counts[idx + 1..][..new_cards as usize].iter().sum::<u32>();
        }
        counts.into_iter().sum::<u32>()
    };

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
