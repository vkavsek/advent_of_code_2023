advent_of_code::solution!(2);

struct Game {
    colors: (usize, usize, usize),
}

impl Game {
    fn new(r: usize, g: usize, b: usize) -> Game {
        Game { colors: (r, g, b) }
    }

    fn update_color(&mut self, color: &str, number: usize) {
        match color {
            "red" => self.colors.0 = self.colors.0.max(number),
            "green" => self.colors.1 = self.colors.1.max(number),
            "blue" => self.colors.2 = self.colors.2.max(number),
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
                    let (num, color) = color_data.trim().split_once(' ').unwrap();
                    let num = num.parse::<usize>().expect("couldn't parse a number");

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
        .filter(|(_, game)| game.colors.0 <= 12 && game.colors.1 <= 13 && game.colors.2 <= 14)
        .map(|(i, _)| i + 1)
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let games = get_games(input);

    let res = games
        .into_iter()
        .map(|game| game.colors.0 * game.colors.1 * game.colors.2)
        .sum();

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
