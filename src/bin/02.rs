advent_of_code::solution!(2);

struct Game {
    r: usize,
    g: usize,
    b: usize,
}

impl Game {
    fn new(r: usize, g: usize, b: usize) -> Game {
        Game { r, g, b }
    }

    fn update_color(&mut self, color: &str, number: usize) {
        match color {
            "red" => {
                if number > self.r {
                    self.r = number;
                }
            }
            "green" => {
                if number > self.g {
                    self.g = number;
                }
            }
            "blue" => {
                if number > self.b {
                    self.b = number;
                }
            }
            _ => {}
        }
    }
}

fn get_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut game = Game::new(0, 0, 0);
            let (_game, data) = line.split_once(':').expect("error parsing input");
            for data_chunk in data.split(';') {
                for color_data in data_chunk.split(',') {
                    let color_data = color_data.trim();
                    let (num, color) = color_data.split_once(' ').unwrap();
                    let num = num.parse::<usize>().expect("Couldn't pass a number");

                    game.update_color(color, num);
                }
            }
            game
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let games = get_games(input);

    let res: usize = games
        .into_iter()
        .enumerate()
        .filter(|(_, game)| game.r <= 12 && game.g <= 13 && game.b <= 14)
        .map(|(i, _)| i + 1)
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let games = get_games(input);

    let res = games.into_iter().map(|game| game.r * game.g * game.b).sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
