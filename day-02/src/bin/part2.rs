fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

#[derive(Debug)]
struct Game {
    power: u32,
}

impl Game {
    fn from(line: &str) -> Self {
        let mut min_blue = 0;
        let mut min_green = 0;
        let mut min_red = 0;

        for draw in line.split(": ").nth(1).unwrap().split("; ") {
            for color in draw.split(", ") {
                let mut parts = color.split_whitespace();
                let count = parts.next().unwrap().parse::<u32>().unwrap();
                let color = parts.next().unwrap();
                match color {
                    "blue" => min_blue = std::cmp::max(min_blue, count),
                    "green" => min_green = std::cmp::max(min_green, count),
                    "red" => min_red = std::cmp::max(min_red, count),
                    _ => panic!("Invalid color"),
                }
            }
        }

        Self {
            power: min_blue * min_green * min_red,
        }
    }
}

fn run(input: &str) -> String {
    let mut game_powers: Vec<u32> = vec![];
    input.lines().for_each(|line| {
        game_powers.push(Game::from(line).power);
    });

    game_powers.iter().sum::<u32>().to_string()
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
        assert_eq!(result, "2286")
    }
}
