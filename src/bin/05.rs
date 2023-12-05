use std::collections::BTreeSet;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::info;

advent_of_code::solution!(5);

#[derive(Clone, Debug)]
struct DataChunk<'ch> {
    data: Vec<&'ch str>,
}
impl<'ch> DataChunk<'ch> {
    fn new() -> Self {
        DataChunk { data: Vec::new() }
    }
    fn clear(&mut self) {
        self.data.clear();
    }
    fn push(&mut self, value: &'ch str) {
        self.data.push(value);
    }
    fn iter(&self) -> std::slice::Iter<'_, &str> {
        self.data.iter()
    }
}

fn parse_to_u32(input: &str) -> u32 {
    input
        .parse::<u32>()
        .expect("Number should be parsable to u32")
}

fn parse_to_u64(input: &str) -> u64 {
    input
        .parse::<u64>()
        .expect("Number should be parsable to u32")
}

fn chunkify(input: &str) -> Vec<DataChunk<'_>> {
    let lines_len = input.lines().count();

    input
        .lines()
        .enumerate()
        .skip(3)
        .fold(
            (Vec::new(), DataChunk::new()),
            |(mut coll, mut chunk), (i, line)| {
                if line.chars().next().unwrap_or_default().is_ascii_digit() {
                    chunk.push(line);
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

// TODO: Make multithreaded
fn proccess_seed(seed: &u32, chunks: &Vec<DataChunk>) -> u32 {
    let mut num_op = *seed as u64;
    for chunk in chunks {
        for &line in chunk.iter() {
            let (dest, src, len) = line
                .split_whitespace()
                .map(parse_to_u64)
                .collect_tuple()
                .expect("There should be 3 numbers on a line!");
            if num_op >= src && num_op < src + len {
                num_op = num_op - src + dest;
                break;
            }
        }
    }
    num_op as u32
}

fn proccess_seeds(seed_map: Vec<u32>, chunks: &Vec<DataChunk>) -> BTreeSet<u32> {
    seed_map
        .iter()
        .enumerate()
        .map(|(_i, seed)| {
            // println!("Processing seed number: {i}");
            let mut num_op = *seed;
            for chunk in chunks {
                for &line in chunk.iter() {
                    let (dest, src, len) = line
                        .split_whitespace()
                        .map(parse_to_u32)
                        .collect_tuple()
                        .expect("There should be 3 numbers on a line!");
                    if num_op >= src && num_op < src + len {
                        num_op = num_op - src + dest;
                        break;
                    }
                }
            }
            num_op
        })
        .collect::<BTreeSet<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let seed_map: Vec<u32> = input
        .lines()
        .take(1)
        .flat_map(|line| {
            let (_, nums) = line.split_once(": ").expect("Should be splitable by ': '");
            nums.split_whitespace().map(parse_to_u32)
        })
        .collect_vec();

    let chunks = chunkify(input);

    let res_map = proccess_seeds(seed_map, &chunks);
    res_map.first().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    tracing_subscriber::fmt().with_target(false).init();

    info!("ENTERED part_two");
    let seed_map = input
        .lines()
        .take(1)
        .flat_map(|line| {
            let (_, nums) = line.split_once(": ").expect("Should be splitable by ': '");
            nums.split_whitespace()
                .map(parse_to_u32)
                .tuples::<(_, _)>()
                .flat_map(|(seed_num, len)| seed_num..seed_num + len)
        })
        .collect_vec();

    info!("Collected seed map - LEN: {}", seed_map.len());
    let chunks = chunkify(input);
    info!("Chunkified - LEN: {}", chunks.len());

    let res_map = seed_map
        .par_iter()
        .map(|seed| proccess_seed(seed, &chunks))
        .collect::<BTreeSet<_>>();
    info!("Seeds processed");
    info!("{:?}", res_map.first());

    res_map.first().map(|n| n.to_owned())
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
