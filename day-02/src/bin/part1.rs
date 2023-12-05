fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

const AVAILABLE_RED: u32 = 12;
const AVAILABLE_GREEN: u32 = 13;
const AVAILABLE_BLUE: u32 = 14;

enum Error {
    InvalidDraw,
}

#[derive(Debug)]
struct Game {
    id: u32,
}

impl Game {
    fn try_from(line: &str) -> Result<Self, Error> {
        let id = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(':')
            .nth(0)
            .unwrap();

        for draw in line.split(": ").nth(1).unwrap().split("; ") {
            for color in draw.split(", ") {
                let mut parts = color.split_whitespace();
                let count = parts.next().unwrap().parse::<u32>().unwrap();
                let color = parts.next().unwrap();
                match color {
                    "blue" => {
                        if count > AVAILABLE_BLUE {
                            return Err(Error::InvalidDraw);
                        }
                    }
                    "green" => {
                        if count > AVAILABLE_GREEN {
                            return Err(Error::InvalidDraw);
                        }
                    }
                    "red" => {
                        if count > AVAILABLE_RED {
                            return Err(Error::InvalidDraw);
                        }
                    }
                    _ => panic!("Invalid color"),
                }
            }
        }

        Ok(Self {
            id: id.parse::<u32>().unwrap(),
        })
    }
}

fn run(input: &str) -> String {
    let mut valid_game_ids: Vec<u32> = vec![];
    input.lines().for_each(|line| {
        if let Ok(game) = Game::try_from(line) {
            valid_game_ids.push(game.id);
        }
    });

    valid_game_ids.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "8")
    }
}
