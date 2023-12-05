use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug, Clone)]
struct DataMap {
    mappings: BTreeMap<u32, u32>,
}
impl DataMap {
    fn fill_datamap(&mut self, fill_to: usize) {
        for i in 0..=fill_to {
            self.mappings.entry(i as u32).or_insert(i as u32);
        }
    }
}

#[derive(Debug, Clone)]
struct Almanach {
    data_maps: Vec<DataMap>,
}
impl Almanach {
    // Initialize an Almanach with empty DataMap inside.
    fn init() -> Almanach {
        Almanach {
            data_maps: vec![DataMap {
                mappings: BTreeMap::new(),
            }],
        }
    }
    fn push_new_dm(&mut self) {
        self.data_maps.push(DataMap {
            mappings: BTreeMap::new(),
        });
    }

    /// Inserts into the latest created DataMap
    /// If data_maps Vec is empty, this function returns None.
    /// Insertion is infallible, if the key already exists the value
    /// is updated and Some(key) is returned.
    fn insert_into_dm(&mut self, key_src: u32, value_dest: u32) -> Option<u32> {
        self.data_maps
            .last_mut()?
            .mappings
            .entry(key_src)
            .or_insert(value_dest);
        Some(key_src)
    }

    /// Returns None if it can't find a KEY_SRC entry in any of the DataMaps.
    fn find_final_elem(&mut self, n: u32) -> Option<u32> {
        let mut key_src = n;
        let mut data_iter = self.data_maps.iter_mut().peekable();
        while let Some(data_map) = data_iter.next() {
            let dest = data_map
                .mappings
                .get(&key_src)
                .unwrap_or(&key_src)
                .to_owned();
            if let Some(next_data_map) = data_iter.peek_mut() {
                next_data_map.fill_datamap(dest as usize);
            }

            key_src = dest;
        }
        Some(key_src)
    }
}

fn parse_to_u32(input: &str) -> u32 {
    input
        .parse::<u32>()
        .expect("Number should be parsable to u32")
}

fn process_line(alm: &mut Almanach, line: &str) {
    match line {
        l if l.is_empty() => {
            alm.push_new_dm();
        }
        l if l.chars().next().unwrap_or_default().is_ascii_digit() => {
            let (dest, src, len) = l
                .split_whitespace()
                .map(parse_to_u32)
                .collect_tuple()
                .expect("3 numbers should be present per line!");
            for i in 0..len {
                alm.insert_into_dm(src + i, dest + i);
            }
        }
        _ => {}
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut alm = Almanach::init();
    let seed_map: Vec<u32> = input
        .lines()
        .take(1)
        .flat_map(|line| {
            let (_, nums) = line.split_once(": ").expect("Should be splitable by ': '");
            nums.split_whitespace().map(parse_to_u32)
        })
        .collect_vec();

    for line in input.lines().skip(3) {
        process_line(&mut alm, line);
    }

    // println!("{:?}", &alm);
    let btree = seed_map
        .iter()
        .map(|seed| {
            alm.find_final_elem(*seed)
                .unwrap_or_else(|| panic!("couldn't find the KEY_SRC for SEED: {}", seed))
        })
        .collect::<BTreeSet<_>>();

    btree.first().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let _ = input;
    None
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
        assert_eq!(result, None);
    }
}
