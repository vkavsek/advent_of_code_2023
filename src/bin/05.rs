use std::{cell::RefCell, collections::BTreeSet, ops::Range, rc::Rc};

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::info;

advent_of_code::solution!(5);

#[derive(Clone, Debug)]
struct DataChunk {
    data: Vec<(Range<u64>, Range<u64>)>,
}
impl DataChunk {
    fn new() -> Self {
        DataChunk { data: Vec::new() }
    }
    fn clear(&mut self) {
        self.data.clear();
    }
    fn push(&mut self, value: (Range<u64>, Range<u64>)) {
        self.data.push(value);
    }
    fn iter(&self) -> std::slice::Iter<'_, (Range<u64>, Range<u64>)> {
        self.data.iter()
    }
}

fn parse_to_u64(input: &str) -> u64 {
    input
        .parse::<u64>()
        .expect("Number should be parsable to u64")
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
                        .map(parse_to_u64)
                        .collect_tuple()
                        .expect("There should be 3 numbers on a line!");
                    let src_range = src..src + len;
                    let dest_range = dest..dest + len;
                    chunk.push((src_range, dest_range));
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

fn proccess_seed(seed: &u64, chunks: &Vec<DataChunk>) -> u64 {
    let mut num_op = *seed;
    for chunk in chunks {
        let valid_map = chunk
            .iter()
            .find(|(src_range, _dest_range)| src_range.contains(&num_op));
        if let Some((src_range, dest_range)) = valid_map {
            num_op = dest_range.start + num_op - src_range.start;
        }
    }
    num_op
}

fn proccess_seeds(seed_map: Vec<u64>, chunks: &Vec<DataChunk>) -> BTreeSet<u64> {
    seed_map
        .iter()
        .map(|seed| {
            info!("changing seed");
            let mut num_op = *seed;
            for chunk in chunks {
                let valid_map = chunk
                    .iter()
                    .find(|(src_range, _dest_range)| src_range.contains(&num_op));
                if let Some((src_range, dest_range)) = valid_map {
                    num_op = dest_range.start + num_op - src_range.start;
                }
            }
            num_op
        })
        .collect::<BTreeSet<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let seed_map: Vec<u64> = input
        .lines()
        .take(1)
        .flat_map(|line| {
            let (_, nums) = line.split_once(": ").expect("Should be splitable by ': '");
            nums.split_whitespace().map(parse_to_u64)
        })
        .collect_vec();

    let chunks = chunkify(input);
    info!(?chunks);

    let res_map = proccess_seeds(seed_map, &chunks);
    res_map.first().map(|n| n.to_owned() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    tracing_subscriber::fmt().with_target(false).init();

    info!("ENTERED part_two");
    let count = Rc::new(RefCell::new(0));

    let seed_map = input
        .lines()
        .take(1)
        .flat_map(|line| {
            let (_, nums) = line.split_once(": ").expect("Should be splitable by ': '");
            nums.split_whitespace()
                .map(parse_to_u64)
                .tuples::<(_, _)>()
                .map(|(seed_num, len)| {
                    let mut c = count.borrow_mut();
                    *c = len;
                    seed_num..seed_num + len
                })
        })
        .collect_vec();
    info!("Collected seed map - LEN: {}", seed_map.len());

    let chunks = chunkify(input);
    info!("Chunkified - LEN: {}", chunks.len());

    let res = seed_map
        .par_iter()
        .progress_count(*count.borrow())
        .flat_map(|range| range.clone())
        .map(|seed| proccess_seed(&seed, &chunks))
        .min();
    info!("Seeds processed");

    res.map(|n| n as u32)
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
        assert_eq!(result, Some(46));
    }
}
