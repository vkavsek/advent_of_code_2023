use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

#[derive(Debug)]
struct NumberData {
    number: Vec<char>,
    is_part: bool,
}
impl NumberData {
    fn new() -> Self {
        NumberData {
            number: Vec::new(),
            is_part: false,
        }
    }
    fn push_num(&mut self, ch: char) {
        self.number.push(ch);
    }
    fn is_empty(&self) -> bool {
        self.number.is_empty()
    }
    fn extract_num(&self) -> usize {
        self.number
            .iter()
            .collect::<String>()
            .parse()
            .expect("should be a number")
    }
    fn clear(&mut self) {
        self.number.clear();
        self.is_part = false;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = lines.len();
    let columns = lines[0].len();

    let mut numbers = Vec::new();

    for (y_id, line) in input.lines().enumerate() {
        let mut number = NumberData::new();
        for (x_id, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                number.push_num(ch);

                if !number.is_part {
                    let lower_y = if y_id > 0 { y_id - 1 } else { y_id };
                    let lower_x = if x_id > 0 { x_id - 1 } else { x_id };
                    let upper_y = if y_id < rows - 1 { y_id + 1 } else { y_id };
                    let upper_x = if x_id < columns - 1 { x_id + 1 } else { x_id };

                    for y in lower_y..=upper_y {
                        for x in lower_x..=upper_x {
                            if let Some(line) = lines.get(y) {
                                if let Some(&chr) = line.get(x) {
                                    if !chr.is_ascii_digit() && chr != '.' {
                                        number.is_part = true;
                                    }
                                }
                            }
                        }
                    }
                }
            } else if !number.is_empty() {
                numbers.push(number);
                number = NumberData::new();
            }
            if x_id == columns - 1 && !number.is_empty() {
                numbers.push(number);
                number = NumberData::new();
            }
        }
    }

    let res: usize = numbers
        .iter()
        .filter(|number| number.is_part)
        .map(|number| number.extract_num())
        .sum();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = lines.len();
    let columns = lines[0].len();

    let mut gears_n_nums: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for (y_id, line) in input.lines().enumerate() {
        let mut number = NumberData::new();
        let mut num_specific_g = HashSet::new();

        for (x_id, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                number.push_num(ch);

                if !number.is_part {
                    let lower_y = if y_id > 0 { y_id - 1 } else { y_id };
                    let lower_x = if x_id > 0 { x_id - 1 } else { x_id };
                    let upper_y = if y_id < rows - 1 { y_id + 1 } else { y_id };
                    let upper_x = if x_id < columns - 1 { x_id + 1 } else { x_id };

                    for y in lower_y..=upper_y {
                        for x in lower_x..=upper_x {
                            if let Some(line) = lines.get(y) {
                                if let Some(&chr) = line.get(x) {
                                    if chr == '*' {
                                        num_specific_g.insert((x, y));
                                    }
                                }
                            }
                        }
                    }
                }
            } else if !number.is_empty() {
                clear_and_collect(&mut number, &mut num_specific_g, &mut gears_n_nums);
            }
            if x_id == columns - 1 && !number.is_empty() {
                clear_and_collect(&mut number, &mut num_specific_g, &mut gears_n_nums);
            }
        }
    }

    let res: usize = gears_n_nums
        .values()
        .filter(|gear_val| gear_val.len() == 2)
        .map(|gear_val| gear_val[0] * gear_val[1])
        .sum();

    Some(res as u32)
}

fn clear_and_collect(
    number: &mut NumberData,
    num_specific_g: &mut HashSet<(usize, usize)>,
    gears_n_nums: &mut HashMap<(usize, usize), Vec<usize>>,
) {
    for gear in num_specific_g.iter() {
        if !gears_n_nums.contains_key(gear) {
            gears_n_nums.insert(*gear, Vec::new());
        }
        if let Some(entry) = gears_n_nums.get_mut(gear) {
            entry.push(number.extract_num());
        }
    }
    num_specific_g.clear();
    number.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
