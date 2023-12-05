fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

fn run(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            let first_char = numbers.chars().next().unwrap();
            let last_char = numbers.chars().last().unwrap();
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
        let result = run("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
");
        assert_eq!(result, "142")
    }
}
