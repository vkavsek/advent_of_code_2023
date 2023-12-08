use itertools::Itertools;
use rustc_hash::FxHashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Directions {
    left: u32,
    right: u32,
}
impl Directions {
    fn new(left: u32, right: u32) -> Directions {
        Directions { left, right }
    }
}

type DirTable = FxHashMap<u32, Directions>;

fn map_element(elem: &str) -> u32 {
    let (x, y, z) = elem.bytes().collect_tuple().expect("3 chars");
    format!("{x}{y}{z}").parse::<u32>().expect("parsable num")
}

fn line_to_nums(line: &str) -> (u32, Directions) {
    let (id, left_right) = line.split_once(" = ").expect("valid parse");
    let (left, right) = left_right
        .trim_matches(|c| c == '(' || c == ')')
        .split_once(", ")
        .expect("valid parse");

    (
        map_element(id),
        Directions::new(map_element(left), map_element(right)),
    )
}

static START: u32 = 656565; // -> "AAA"
static GOAL: u32 = 909090; // -> "ZZZ"

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, directions) = input.split_once("\n\n").expect("valid parse");
    let dir_table: DirTable = directions.lines().map(line_to_nums).collect();

    let mut find_key = START;
    let mut acc = 0;
    loop {
        if find_key == GOAL {
            break;
        }
        for inst in instructions.chars() {
            acc += 1;
            match inst {
                'R' => {
                    find_key = dir_table
                        .get(&find_key)
                        .expect("key should be present")
                        .right;
                }
                'L' => {
                    find_key = dir_table
                        .get(&find_key)
                        .expect("key should be present")
                        .left;
                }
                _ => panic!("should never happen"),
            }
        }
    }
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
