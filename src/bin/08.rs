use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Directions<'a> {
    left: &'a str,
    right: &'a str,
}
impl<'a> Directions<'a> {
    fn new(left: &'a str, right: &'a str) -> Directions<'a> {
        Directions { left, right }
    }
}

type DirTable<'a> = FxHashMap<&'a str, Directions<'a>>;

fn process_line(line: &str) -> (&str, Directions) {
    let (id, left_right) = line.split_once(" = ").expect("valid parse");
    let (left, right) = left_right
        .trim_matches(|c| c == '(' || c == ')')
        .split_once(", ")
        .expect("valid parse");

    (id, Directions::new(left, right))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, directions) = input.split_once("\n\n").expect("valid parse");
    let dir_table: DirTable = directions.lines().map(process_line).collect();

    let mut find_key = "AAA";
    let mut acc = 0;
    loop {
        if find_key == "ZZZ" {
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

// PART TWO

#[derive(Debug)]
struct DirectionsTwo {
    left: u32,
    right: u32,
}
impl DirectionsTwo {
    fn new(left: u32, right: u32) -> DirectionsTwo {
        DirectionsTwo { left, right }
    }
}

type DirTableTwo = FxHashMap<u32, DirectionsTwo>;

fn map_element(elem: &str) -> u32 {
    let (x, y, z) = elem.bytes().collect_tuple().expect("3 chars");
    format!("{x}{y}{z}").parse::<u32>().expect("parsable num")
}

fn process_line_two(line: &str) -> (u32, DirectionsTwo) {
    let (id, left_right) = line.split_once(" = ").expect("valid parse");
    let (left, right) = left_right
        .trim_matches(|c| c == '(' || c == ')')
        .split_once(", ")
        .expect("valid parse");

    (
        map_element(id),
        DirectionsTwo::new(map_element(left), map_element(right)),
    )
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }

    let mut r = a % b;
    let mut a;
    let mut b = b / r * r + b % r;
    while r > 0 {
        a = b;
        b = r;
        r = a % b;
    }
    b
}

fn lcm(a: usize, b: usize) -> usize {
    let gcd = gcd(a, b);
    a * (b / gcd)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (instructions, directions) = input.split_once("\n\n").expect("valid parse");
    let (mut starting, mut ending) = (Vec::new(), Vec::new());
    let dir_table: DirTableTwo = directions
        .lines()
        .map(process_line_two)
        .inspect(|(id, _)| {
            if (id - 65) % 100 == 0 {
                starting.push(*id);
            } else if (id - 90) % 100 == 0 {
                ending.push(*id);
            }
        })
        .collect();

    let find_keys = starting;
    let mut res_col = Vec::with_capacity(find_keys.len());
    find_keys
        .par_iter()
        .map(|find_key| {
            let mut find_key = *find_key;
            let mut acc = 0usize;
            'outer: loop {
                for inst in instructions.chars() {
                    if (find_key - 90) % 100 == 0 {
                        break 'outer;
                    }
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
            acc
        })
        .collect_into_vec(&mut res_col);

    let mut op = res_col[0];
    for r in res_col.iter().skip(1) {
        op = lcm(op, *r)
    }

    Some(op)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(2));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
