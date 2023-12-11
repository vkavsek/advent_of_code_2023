use itertools::Itertools;

advent_of_code::solution!(9);

fn compute(to_eval: &[i64]) -> (i64, i64) {
    assert!(to_eval.len() == 3);
    (to_eval[1] - to_eval[0], to_eval[2] - to_eval[1])
}

fn solve_one(line: &[i64]) -> i64 {
    let last = line.len() - 1;
    let mut result = Vec::new();
    result.push(line[last]);
    let mut offset = 2;
    while offset <= last {
        // TODO: get rid of this allocation
        let mut to_eval = line[(last - offset)..].to_vec();
        if to_eval.len() > 3 {
            let curr_len = to_eval.len();
            for i in 0..(curr_len - 3) {
                let lower_line = to_eval
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(id, n)| n - to_eval[id - 1])
                    .collect::<Vec<i64>>();
                to_eval = lower_line;
                if i as i32 + 3 - offset as i32 == 0 {
                    let l = to_eval.len();
                    result.push(to_eval[l - 1]);
                }
            }
        }
        let (a, b) = compute(&to_eval);
        if b - a == 0 {
            result.push(b);
            break;
        }
        offset += 1;
    }
    result.iter().sum()
}

fn solve_two(line: &[i64]) -> i64 {
    let last = line.len() - 1;
    let mut n_iters = 1;
    let mut offset = 2;
    while offset <= last {
        // TODO: get rid of this allocation
        let mut to_eval = line[(last - offset)..].to_vec();
        if to_eval.len() > 3 {
            let curr_len = to_eval.len();
            for i in 0..(curr_len - 3) {
                let lower_line = to_eval
                    .iter()
                    .enumerate()
                    .skip(1)
                    .map(|(id, n)| n - to_eval[id - 1])
                    .collect_vec();
                to_eval = lower_line;
                if i as i32 + 3 - offset as i32 == 0 {
                    n_iters += 1;
                }
            }
        }
        let (a, b) = compute(&to_eval);
        if b - a == 0 {
            n_iters += 1;
            break;
        }
        offset += 1;
    }
    let mut main_eval = line[..n_iters + 1].to_vec();
    let mut result = Vec::new();
    result.push(line[0]);
    for _ in 0..n_iters - 1 {
        main_eval = main_eval
            .iter()
            .enumerate()
            .skip(1)
            .map(|(id, n)| n - main_eval[id - 1])
            .collect_vec();
        result.push(main_eval[0]);
    }
    let mut diff = 0;
    for res in result.iter().rev() {
        diff = res - diff;
    }
    diff
}

fn to_i64(n: &str) -> i64 {
    n.parse().expect("should be a valid i64")
}

pub fn part_one(input: &str) -> Option<i64> {
    let r = input
        .lines()
        .map(|line| line.split_whitespace().map(to_i64).collect_vec())
        .map(|line| solve_one(&line))
        .sum();
    Some(r)
}

pub fn part_two(input: &str) -> Option<i64> {
    let r = input
        .lines()
        .map(|line| line.split_whitespace().map(to_i64).collect_vec())
        .map(|line| solve_two(&line))
        .sum();
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

// NOTE:
// let line = input;
// let last = line.len() - 1;
// vec Result
// Result.push((line[last - 1], line[last]));
// let offset = 2;
// loop {
//      let mut to_eval = line[(last - offset)..];
//      if to_eval.len() > 3 {
//          let curr_len = to_eval.len();
//          for i in 0..(curr_len - 3) {
//              let lower_line = to_eval.iter().enumerate().skip(1)
//                  .map(|(i, n) n - to_eval[i])
//                  .collect();
//              if i + 3 - offset == 0 {
//                  let l = to_eval.len();
//                  result.push( (to_eval[l-2], to_eval[l-1]) );
//              }
//              to_eval = lower_line;
//          }
//      }
//      let (a, b) = to_eval.compute();
//      if b - a == 0 {
//          result.push(b);
//          result.push(0);
//          break;
//      }
//      offset += 1;
//  }
//
//
// len of to_eval - 2 = iterations
//
// let (g, h)  = compute(x, y, z);
// if h - g == 0 {
//      push h;
//      push 0;
//  } else {
//      push h;
//  }
//
//  x = ()
//
//
//
//
//
//
