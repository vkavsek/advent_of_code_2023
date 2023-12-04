use std::collections::{BTreeMap, HashMap};

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
    let mut hm_res: BTreeMap<usize, u32> = BTreeMap::new();
    let mut tt = Vec::new();

    for (i, line) in input.lines().enumerate() {
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

        tt.push(card_num);

        hm_res.entry(i).and_modify(|e| *e += 1).or_insert(1); // Don't move from in-front of the FORloop
        let lower = i + 1;
        let higher = lower + card_num as usize;
        let add = hm_res.get(&i).unwrap_or(&1).to_owned();

        for it in lower..higher {
            hm_res.entry(it).and_modify(|e| *e += add).or_insert(1);
        }
    }

    let res = hm_res.values().sum();

    println!("{:?}", tt);
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
