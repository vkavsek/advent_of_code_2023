use itertools::Itertools;
use rustc_hash::FxHashMap;

advent_of_code::solution!(7);

type Hands = Vec<Hand>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    rank: u8,
    strength: usize,
    bid: usize,
}
impl Hand {
    fn new(rank: u8, strength: usize, bid: usize) -> Self {
        Hand {
            rank,
            strength,
            bid,
        }
    }
}

fn parse_to_usize(n: &str) -> usize {
    n.parse().expect("a valid usize number")
}

fn card_val(card: char) -> usize {
    match card {
        c if c.is_ascii_digit() => c.to_digit(10).unwrap() as usize - 1,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

fn get_rank_strength(cards: &str) -> (u8, usize) {
    let mut acc = FxHashMap::default();
    let mut strength = 0;
    for (i, card) in cards.chars().rev().enumerate() {
        acc.entry(card).and_modify(|val| *val += 1).or_insert(1);
        strength += card_val(card) * (10usize.pow((i as u32 + 1) * 2));
    }

    let rank = match acc.len() {
        1 => 6,
        2 => {
            if acc.values().contains(&4) {
                5
            } else {
                4
            }
        }
        3 => {
            if acc.values().contains(&3) {
                3
            } else {
                2
            }
        }

        4 => 1,
        _ => 0,
    };

    (rank, strength)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut hands: Hands = input
        .lines()
        .map(|line| {
            let split_id = line.find(' ').expect("a valid parse");
            let (cards, bid) = (&line[0..split_id], parse_to_usize(&line[split_id + 1..]));
            let (rank, strength) = get_rank_strength(cards);
            Hand::new(rank, strength, bid)
        })
        .collect_vec();
    hands.sort_unstable();

    let res = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum();

    Some(res)
}

fn card_val_2(card: char) -> usize {
    match card {
        c if c.is_ascii_digit() => c.to_digit(10).unwrap() as usize,
        'J' => 1,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

fn get_rank_strength_2(cards: &str) -> (u8, usize) {
    let mut acc = FxHashMap::default();
    let mut strength = 0;
    let mut j = 0;
    for (i, card) in cards.chars().rev().enumerate() {
        if card == 'J' {
            j += 1;
        } else {
            acc.entry(card).and_modify(|val| *val += 1).or_insert(1u32);
        }
        strength += card_val_2(card) * (10usize.pow((i as u32 + 1) * 2));
    }

    if let Some(highest) = acc.values_mut().max() {
        *highest += j;
    }

    let rank = match acc.len() {
        1 => 6,
        2 => {
            if acc.values().contains(&4) {
                5
            } else {
                4
            }
        }
        3 => {
            if acc.values().contains(&3) {
                3
            } else {
                2
            }
        }

        4 => 1,
        0 => 6,
        _ => 0,
    };

    (rank, strength)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut hands: Hands = input
        .lines()
        .map(|line| {
            let split_id = line.find(' ').expect("a valid parse");
            let (cards, bid) = (&line[0..split_id], parse_to_usize(&line[split_id + 1..]));
            let (rank, strength) = get_rank_strength_2(cards);
            Hand::new(rank, strength, bid)
        })
        .collect_vec();
    hands.sort_unstable();

    let res = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
