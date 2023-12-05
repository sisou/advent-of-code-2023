fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

fn run(input: &str) -> String {
    let spelled_out_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            // Store position and number of found numbers
            let mut numbers: Vec<(usize, u32)> = vec![];

            // First find actual numbers and their positions
            line.chars().enumerate().for_each(|(i, c)| {
                if c.is_ascii_digit() {
                    numbers.push((i, c.to_string().parse::<u32>().unwrap()));
                }
            });

            // Then find spelled out numbers and their positions
            spelled_out_numbers
                .iter()
                .enumerate()
                .for_each(|(j, number)| {
                    let mut start = 0;
                    while let Some(i) = line[start..].find(number) {
                        numbers.push((start + i, (j + 1) as u32));
                        start += i + number.len();
                    }
                });

            // Sort by position
            numbers.sort_by(|a, b| a.0.cmp(&b.0));

            // Combine the first and last number into a string and then parse it as u32
            let first_char = numbers.first().unwrap().1.to_string();
            let last_char = numbers.last().unwrap().1.to_string();
            format!("{}{}", first_char, last_char)
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
");
        assert_eq!(result, "281")
    }
}
