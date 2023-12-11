use std::{
    cell::RefCell,
    ops::{Add, AddAssign, RangeInclusive, Sub},
    rc::Rc,
};

use itertools::Itertools;
use range_ext::intersect::{Intersect, IntersectionExt};

advent_of_code::solution!(10);

#[derive(Clone, Copy, PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
    fn update(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    // Each line has 140 + 1 ('\n') chars.
    // '\n' chars don't get collected into a vector.
    fn find_s_from_input(pos_in_str: usize, line_len: usize) -> Self {
        let pos_in_str = pos_in_str as i32;
        let line_len = line_len as i32;
        Self::new(pos_in_str % (line_len + 1), pos_in_str / (line_len + 1))
    }
}
impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn char_to_move(ch: &char, movement: Coord) -> Option<Coord> {
    // (Coord-'NATURAL' movement, Coord_'UNNATURAL' movement)
    let mut coord = Coord::new(0, 0);
    match ch {
        '|' => Some(movement),
        '-' => Some(movement),
        'L' | '7' => {
            coord.update(movement.y, movement.x);
            Some(coord)
        }
        'F' | 'J' => {
            coord.update(-movement.y, -movement.x);
            Some(coord)
        }
        'S' => Some(coord),
        _ => None,
    }
}

const DIRS_INPUTS: [(Coord, &str); 4] = [
    (Coord { x: -1, y: 0 }, "-LF"),
    (Coord { x: 1, y: 0 }, "-7J"),
    (Coord { x: 0, y: -1 }, "|7F"),
    (Coord { x: 0, y: 1 }, "|JL"),
];

fn find_pipe(map: &[Vec<char>], s_pos: Coord) -> Option<Coord> {
    for (dir, valid_inputs) in DIRS_INPUTS {
        let new_pos = s_pos + dir;
        let x: Result<usize, _> = new_pos.x.try_into();
        let y: Result<usize, _> = new_pos.y.try_into();
        let (Ok(x), Ok(y)) = (x, y) else {
            continue;
        };
        if valid_inputs.contains(map[y][x]) {
            return Some(new_pos);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let s_pos = Coord::find_s_from_input(input.find('S')?, map[0].len());

    let mut last_pos = s_pos;
    let mut curr_pos = find_pipe(&map, s_pos)?;
    let mut ch = map[curr_pos.y as usize][curr_pos.x as usize];

    let mut r = Vec::with_capacity(16384);
    r.push('S');
    while ch != 'S' {
        //
        r.push(ch);
        let movement = curr_pos - last_pos;
        last_pos = curr_pos;
        let next_move = char_to_move(&ch, movement)?;
        curr_pos = last_pos + next_move;
        ch = map[curr_pos.y as usize][curr_pos.x as usize];
    }
    Some(r.len() / 2)
}

fn collect_ranges_and_validate_positions(
    valid_positions: &mut [Vec<bool>],
    s_pos: Coord,
    map: &[Vec<char>],
) -> Vec<Vec<RangeInclusive<i32>>> {
    let mut ranges: Vec<Vec<RangeInclusive<i32>>> =
        map.iter().map(|_| Vec::with_capacity(256)).collect_vec();

    valid_positions[s_pos.y as usize][s_pos.x as usize] = false;

    let mut last_pos = s_pos;
    let mut curr_pos = find_pipe(map, s_pos).expect("to find a connected pipe");
    let mut ch = map[curr_pos.y as usize][curr_pos.x as usize];
    let mut curr_start_range = s_pos.x;
    let mut curr_end_range = s_pos.x;
    let mut previous_move = curr_pos - last_pos;

    while ch != 'S' {
        ch = map[curr_pos.y as usize][curr_pos.x as usize];
        valid_positions[curr_pos.y as usize][curr_pos.x as usize] = false;
        let movement = curr_pos - last_pos;
        match movement {
            Coord { x: 1, .. } => {
                curr_end_range = curr_pos.x;
                if ch == 'S' {
                    ranges[curr_pos.y as usize].push(curr_start_range..=curr_end_range);
                }
            }
            Coord { x: -1, .. } => {
                curr_start_range = curr_pos.x;
                if ch == 'S' {
                    ranges[curr_pos.y as usize].push(curr_start_range..=curr_end_range);
                }
            }
            Coord { y: 1, .. } | Coord { y: -1, .. } => {
                if previous_move.x.abs() > 0 {
                    ranges[last_pos.y as usize].push(curr_start_range..=curr_end_range);
                    // PUSH
                }
                curr_start_range = curr_pos.x;
                curr_end_range = curr_pos.x;
                // ranges[curr_pos.y as usize].push(curr_start_range..=curr_end_range);
            }
            _ => {}
        }
        previous_move = movement;
        let next_move = char_to_move(&ch, movement).expect("a valid move");
        last_pos = curr_pos;
        curr_pos = last_pos + next_move;
    }
    ranges
}

fn try_range_combine(
    range: &RangeInclusive<i32>,
    previous_ranges: &[RangeInclusive<i32>],
) -> Option<(RangeInclusive<i32>, usize, usize)> {
    let mut new_range = None;

    let (mut id_1, mut id_2) = (0, 0);
    'main_loop: for (upper_range_id, up_range) in previous_ranges.iter().enumerate() {
        for (up_range_two_id, up_range_two) in
            previous_ranges.iter().enumerate().skip(upper_range_id + 1)
        {
            id_1 = upper_range_id;
            id_2 = up_range_two_id;
            if range.start() == up_range.end() && range.end() == up_range_two.start() {
                let (start, end) = (up_range.start().to_owned(), up_range_two.end().to_owned());
                new_range = Some(start..=end);
                break 'main_loop;
            } else if range.end() == up_range.start() && range.start() == up_range_two.end() {
                let (start, end) = (up_range_two.start().to_owned(), up_range.end().to_owned());
                new_range = Some(start..=end);
                break 'main_loop;
            }
        }
    }
    Some((new_range?, id_1, id_2))
}

fn try_range_extend(
    range: &RangeInclusive<i32>,
    previous_ranges: &[RangeInclusive<i32>],
) -> Option<(RangeInclusive<i32>, usize)> {
    let mut new_range = None;

    previous_ranges
        .iter()
        .enumerate()
        .for_each(
            |(up_rang_id, up_range)| match range.intersect_ext(up_range) {
                IntersectionExt::LessOverlap => {
                    new_range = Some((
                        range.start().to_owned()..=up_range.end().to_owned(),
                        up_rang_id,
                    ))
                }
                IntersectionExt::GreaterOverlap => {
                    new_range = Some((
                        up_range.start().to_owned()..=range.end().to_owned(),
                        up_rang_id,
                    ))
                }
                _ => {}
            },
        );
    new_range
}
fn combine_ranges(ranges: &mut Vec<RangeInclusive<i32>>) {
    let mut new_ranges = Vec::with_capacity(ranges.len());
    let mut ids_to_del = Vec::with_capacity(ranges.len());
    ranges.iter().enumerate().for_each(|(prim_r_id, prim_r)| {
        ranges
            .iter()
            .enumerate()
            .skip(prim_r_id + 1)
            .for_each(|(sec_r_id, sec_r)| match prim_r.intersect_ext(sec_r) {
                IntersectionExt::LessOverlap => {
                    let new_range = prim_r.start().to_owned()..=sec_r.end().to_owned();
                    new_ranges.push(new_range);
                    ids_to_del.push((prim_r_id, sec_r_id));
                }
                IntersectionExt::GreaterOverlap => {
                    let new_range = sec_r.start().to_owned()..=prim_r.end().to_owned();
                    new_ranges.push(new_range);
                    ids_to_del.push((prim_r_id, sec_r_id));
                }
                _ => {}
            });
    });
    let mut id_offset = 0;
    for id in ids_to_del {
        ranges.remove(id.0 - id_offset);
        id_offset += 1;
        ranges.remove(id.1 - id_offset);
        id_offset += 1;
    }
    ranges.extend_from_slice(&new_ranges);
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let s_pos = Coord::find_s_from_input(input.find('S')?, map[0].len());

    let mut valid_positions = map
        .iter()
        .map(|line| line.iter().map(|_| true).collect_vec())
        .collect_vec();

    let mut ranges = collect_ranges_and_validate_positions(&mut valid_positions, s_pos, &map);
    ranges.iter_mut().for_each(combine_ranges);

    // let mut res_ranges = Vec::with_capacity(ranges.len());

    let upper_ranges = Rc::new(RefCell::new(
        ranges
            .iter()
            .find(|r| !r.is_empty())
            .cloned()
            .unwrap_or(Vec::default()),
    ));
    let first_pos = ranges
        .iter()
        .find_position(|r| !r.is_empty())
        .map(|(i, _)| i)
        .unwrap_or(0);

    let mut sum = 0;
    for (range_line_id, range_line) in ranges.iter().enumerate().skip(first_pos + 1) {
        let mod_range_line = range_line
            .iter()
            .filter(|lower_range| {
                let combined = try_range_combine(lower_range, &upper_ranges.borrow());
                let extended = try_range_extend(lower_range, &upper_ranges.borrow());
                let collect_check = combined.is_none() && extended.is_none();
                if let Some((new_range, to_del_id_1, to_del_id_2)) = combined {
                    let mut up_rs = upper_ranges.borrow_mut();
                    up_rs.remove(to_del_id_1);
                    up_rs.remove(to_del_id_2 - 1);
                    up_rs.push(new_range);
                } else if let Some((new_range, to_del_id_1)) = extended {
                    let mut up_rs = upper_ranges.borrow_mut();
                    up_rs.remove(to_del_id_1);
                    up_rs.push(new_range);
                }
                collect_check
            })
            .collect_vec();

        'lower_range_loop: for lower_range in mod_range_line {
            let up_rs_hard_clone = upper_ranges.borrow().clone();
            for (up_id, up_r) in up_rs_hard_clone.iter().enumerate() {
                match lower_range.intersect_ext(&up_r.clone()) {
                    IntersectionExt::Same => {
                        let mut up_rs = upper_ranges.borrow_mut();
                        up_rs.remove(up_id);
                        continue 'lower_range_loop;
                    }
                    IntersectionExt::Within => {
                        if lower_range.start() == up_r.start() {
                            let mut up_rs = upper_ranges.borrow_mut();
                            up_rs.remove(up_id);
                            up_rs.push(lower_range.end().to_owned()..=up_r.end().to_owned());
                        } else if lower_range.end() == up_r.end() {
                            let mut up_rs = upper_ranges.borrow_mut();
                            up_rs.remove(up_id);
                            up_rs.push(up_r.start().to_owned()..=lower_range.start().to_owned());
                        } else {
                            let mut up_rs = upper_ranges.borrow_mut();
                            up_rs.remove(up_id);
                            up_rs.push(up_r.start().to_owned()..=lower_range.start().to_owned());
                            up_rs.push(lower_range.end().to_owned()..=up_r.end().to_owned());
                        }
                        continue 'lower_range_loop;
                    }
                    _ => {}
                }
            }
            for up_r in up_rs_hard_clone.iter() {
                match lower_range.intersect_ext(&up_r.clone()) {
                    IntersectionExt::Less | IntersectionExt::Greater => {
                        let mut up_rs = upper_ranges.borrow_mut();
                        up_rs.push(lower_range.to_owned());
                        break;
                    }
                    _ => {}
                }
            }
        }
        for range in upper_ranges.borrow().iter() {
            for i in range.clone() {
                if valid_positions[range_line_id][i as usize] {
                    sum += 1;
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coord_add() {
        let coord_a = Coord::new(10, -15);
        let coord_b = Coord::new(0, 20);
        let result = coord_a + coord_b;
        assert_eq!(result, Coord { x: 10, y: 5 });
    }
    #[test]
    fn test_coord_add_assign() {
        let mut coord_a = Coord::new(10, -15);
        let coord_b = Coord::new(0, 20);
        coord_a += coord_b;
        assert_eq!(coord_a, Coord { x: 10, y: 5 });
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
