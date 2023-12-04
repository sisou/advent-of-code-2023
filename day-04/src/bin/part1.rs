use regex::Regex;

fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

fn run(input: &str) -> String {
    let number_regex = Regex::new("[0-9]+").unwrap();

    let mut points: Vec<u32> = vec![];

    let lines: Vec<_> = input.lines().collect();

    for line in lines.iter() {
        let numbers = line.split(": ").nth(1).unwrap();
        let winning_numbers: Vec<_> = number_regex
            .find_iter(numbers.split(" | ").next().unwrap())
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();
        let my_numbers = number_regex
            .find_iter(numbers.split(" | ").nth(1).unwrap())
            .map(|x| x.as_str().parse::<u32>().unwrap());

        let mut card_points = 0;
        for my_number in my_numbers {
            if winning_numbers.contains(&my_number) {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }
        points.push(card_points);
    }

    points.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, "13")
    }
}
