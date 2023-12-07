use itertools::Itertools;
use rustc_hash::FxHashMap;

advent_of_code::solution!(7);

type Hands = Vec<Hand>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    rank: u8,
    strength: u32,
    bid: u32,
}
impl Hand {
    fn new(rank: u8, strength: u32, bid: u32) -> Self {
        Hand {
            rank,
            strength,
            bid,
        }
    }
}

fn parse_to_u32(n: &str) -> u32 {
    n.parse().expect("a valid usize number")
}

fn card_val(card: &char) -> u32 {
    match *card {
        c if c.is_ascii_digit() => c.to_digit(10).unwrap() - 1,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("should never happen!"),
    }
}

fn get_rank_strength(cards: &str) -> (u8, u32) {
    let mut acc = FxHashMap::default();
    let mut strength = 0;
    for (i, card) in cards.chars().rev().enumerate() {
        acc.entry(card).and_modify(|val| *val += 1).or_insert(1);
        strength += card_val(&card) * (5u32.pow((i as u32 + 1) * 2));
    }

    let rank = match acc.len() {
        1 => 6,
        4 => 1,
        5 => 0,
        2 if acc.values().contains(&4) => 5,
        2 => 4,
        3 if acc.values().contains(&3) => 3,
        3 => 2,
        _ => panic!("should never happen!"),
    };

    (rank, strength)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Hands = input
        .lines()
        .map(|line| {
            let split = line.split_once(' ').expect("a valid parse");
            let (cards, bid) = (split.0, parse_to_u32(split.1));
            let (rank, strength) = get_rank_strength(cards);
            Hand::new(rank, strength, bid)
        })
        .collect_vec();
    hands.sort_unstable();

    let res = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum();

    Some(res)
}

fn card_val_2(card: char) -> u32 {
    match card {
        c if c.is_ascii_digit() => c.to_digit(10).unwrap(),
        'J' => 1,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("should never happen!"),
    }
}

fn get_rank_strength_2(cards: &str) -> (u8, u32) {
    let mut acc = FxHashMap::default();
    let mut strength = 0;
    let mut j = 0;
    for (i, card) in cards.chars().rev().enumerate() {
        if card == 'J' {
            j += 1;
        } else {
            acc.entry(card).and_modify(|val| *val += 1).or_insert(1u32);
        }
        strength += card_val_2(card) * (5u32.pow((i as u32 + 1) * 2));
    }

    if let Some(highest) = acc.values_mut().max() {
        *highest += j;
    }

    let rank = match acc.len() {
        1 => 6,
        4 => 1,
        0 => 6,
        5 => 0,
        2 if acc.values().contains(&4) => 5,
        2 => 4,
        3 if acc.values().contains(&3) => 3,
        3 => 2,
        _ => panic!("should never happen!"),
    };

    (rank, strength)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Hands = input
        .lines()
        .map(|line| {
            let split = line.split_once(' ').expect("a valid parse");
            let (cards, bid) = (split.0, parse_to_u32(split.1));
            let (rank, strength) = get_rank_strength_2(cards);
            Hand::new(rank, strength, bid)
        })
        .collect_vec();
    hands.sort_unstable();

    let res = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
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
