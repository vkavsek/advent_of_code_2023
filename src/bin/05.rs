use std::{
    collections::{BTreeSet, VecDeque},
    ops::Range,
};

use itertools::Itertools;
use range_ext::intersect::{Intersect, IntersectionExt};
use tracing::info;

advent_of_code::solution!(5);

// In part one we have 'small' amount of data in SEEDS so it makes sense to iterate through each SEED.
// Doing it in reverse takes a lot longer since we have a smaller pool to match to.
// Part two BRUTEFORCE counterpart is at the bottom of the file
pub fn part_one(input: &str) -> Option<u32> {
    let seed_map: Vec<i64> = input
        .lines()
        .take(1)
        .flat_map(|line| {
            let (_, nums) = line.split_once(": ").expect("Should be splitable by ': '");
            nums.split_whitespace().map(parse_to_i64)
        })
        .collect_vec();

    let chunks = chunkify(input);
    info!(?chunks);

    let res_map = proccess_seeds(seed_map, &chunks);
    res_map.first().map(|n| n.to_owned() as u32)
}

pub fn part_two(input: &str) -> Option<i64> {
    // tracing_subscriber::fmt().with_target(false).init();
    info!("ENTERED part_two");

    let seed_map = input
        .lines()
        .take(1)
        .flat_map(|line| {
            let (_, nums) = line.split_once(": ").expect("Should be splitable by ': '");
            nums.split_whitespace()
                .map(parse_to_i64)
                .tuples::<(_, _)>()
                .map(|(seed_num, len)| seed_num..seed_num + len)
        })
        .collect::<VecDeque<_>>();
    info!("Collected seed map - LEN: {}", seed_map.len());

    let chunks = chunkify(input);
    info!("Chunkified - LEN: {}", chunks.len());

    process_seed_map(seed_map, &chunks)
}

fn process_seed_map(seed_map: VecDeque<Range<i64>>, chunks: &[DataChunk]) -> Option<i64> {
    let mut seed_ranges = seed_map;
    let mut op_ranges = VecDeque::new();

    for chunk in chunks {
        'seeds: while let Some(mut seed_r) = seed_ranges.pop_front() {
            for (src_r, offset) in &chunk.data {
                match seed_r.intersect_ext(src_r) {
                    IntersectionExt::LessOverlap => {
                        op_ranges.push_back(src_r.start + offset..seed_r.end + offset);
                        seed_r = seed_r.start..src_r.start;
                    }
                    IntersectionExt::GreaterOverlap => {
                        op_ranges.push_back(seed_r.start + offset..src_r.end + offset);
                        seed_r = src_r.end..seed_r.end;
                    }
                    IntersectionExt::Within | IntersectionExt::Same => {
                        op_ranges.push_back(seed_r.start + offset..seed_r.end + offset);
                        continue 'seeds;
                    }
                    IntersectionExt::Over => {
                        let under = seed_r.start..src_r.start;
                        let middle = src_r.start + offset..src_r.end + offset;
                        let over = src_r.end..seed_r.end;
                        op_ranges.push_back(middle);
                        seed_ranges.push_front(under);
                        seed_ranges.push_front(over);
                        continue 'seeds;
                    }
                    _ => {}
                };
            }
            op_ranges.push_back(seed_r);
        }
        seed_ranges = op_ranges;
        op_ranges = VecDeque::new();
    }
    seed_ranges.iter().map(|range| range.start).min()
}

#[derive(Clone, Debug)]
struct DataChunk {
    data: Vec<(Range<i64>, i64)>,
}

impl DataChunk {
    fn new() -> Self {
        DataChunk { data: Vec::new() }
    }
    fn clear(&mut self) {
        self.data.clear();
    }
    fn push(&mut self, value: (Range<i64>, i64)) {
        self.data.push(value);
    }
    fn iter(&self) -> std::slice::Iter<'_, (Range<i64>, i64)> {
        self.data.iter()
    }
}

fn parse_to_i64(input: &str) -> i64 {
    input
        .parse::<i64>()
        .expect("Number should be parsable to i64")
}

fn chunkify(input: &str) -> Vec<DataChunk> {
    let lines_len = input.lines().count();
    input
        .lines()
        .enumerate()
        .skip(3)
        .fold(
            (Vec::new(), DataChunk::new()),
            |(mut coll, mut chunk), (i, line)| {
                if line.chars().next().unwrap_or_default().is_ascii_digit() {
                    let (dest, src, len) = line
                        .split_whitespace()
                        .map(parse_to_i64)
                        .collect_tuple()
                        .expect("There should be 3 numbers on a line!");
                    let src_range = src..src + len;
                    let diff = dest - src;
                    chunk.push((src_range, diff));
                }
                if line.is_empty() || lines_len == i + 1 {
                    coll.push(chunk.clone());
                    chunk.clear();
                }
                (coll, chunk)
            },
        )
        .0
}

fn proccess_seeds(seed_map: Vec<i64>, chunks: &Vec<DataChunk>) -> BTreeSet<i64> {
    seed_map
        .iter()
        .map(|seed| {
            info!("changing seed");
            let mut num_op = *seed;
            for chunk in chunks {
                let valid_map = chunk
                    .iter()
                    .find(|(src_range, _diff)| src_range.contains(&num_op));
                if let Some((_, diff)) = valid_map {
                    num_op += diff;
                }
            }
            num_op
        })
        .collect::<BTreeSet<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // let result_improved =
        //     part_two_improved(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
        // assert_eq!(result, result_improved);
    }
}

// NOTE: This is a demonstration of Bruteforce method used as the original solution.
//
// fn find_seed(chunks: &Vec<DataChunk>, seed_map: &[Range<i64>]) -> Option<u32> {
//     let n = (0u64..u64::MAX).progress_count(u64::MAX).find(|n| {
//         let mut num_op = *n;
//         let mut found = false;
//         for (idx, chunk) in chunks.iter().rev().enumerate() {
//             if let Some((src_range, dest_range)) = chunk
//                 .iter()
//                 .find(|(_, dest_range)| dest_range.contains(&num_op))
//             {
//                 num_op = num_op - dest_range.start + src_range.start;
//             }
//             let seed_map_check = seed_map
//                 .iter()
//                 .any(|seed_range| seed_range.contains(&num_op));
//             // if idx == chunks.len() - 1 && seed_map_check {
//             //     return Some(n as u32);
//             // }
//             if idx == chunks.len() - 1 && seed_map_check {
//                 found = true;
//                 info!("FOUND a match: {n}");
//             }
//         }
//         found
//     });
//     n.map(|n| n as u32)
// }
//
// In part two we have enormous ranges of data in SEEDS so instead of processing each SEED it makes
// sense to BRUTEFORCE in reverse and look for matches (even more so because of the huge ranges,
// there is a big chance we will hit a match early).
//
// pub fn part_two_bruteforce(input: &str) -> Option<u32> {
//     let seed_map = input
//         .lines()
//         .take(1)
//         .flat_map(|line| {
//             let (_, nums) = line.split_once(": ").expect("Should be splitable by ': '");
//             nums.split_whitespace()
//                 .map(parse_to_i64)
//                 .tuples::<(_, _)>()
//                 .map(|(seed_num, len)| seed_num..seed_num + len)
//         })
//         .collect_vec();
//
//     let chunks = chunkify(input);
//     find_seed(&chunks, &seed_map)
// }
