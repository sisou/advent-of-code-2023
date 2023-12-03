use regex::Regex;

fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

fn run(input: &str) -> String {
    let number_regex = Regex::new("[0-9]+").unwrap();
    let symbol_regex = Regex::new("[^0-9.]").unwrap();

    let mut part_numbers: Vec<u32> = vec![];

    let lines: Vec<_> = input.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let numbers = number_regex.find_iter(line);
        for number in numbers {
            // Adjacent symbols, even diagonally, mean that the number is a part number
            let search_start = number.start().saturating_sub(1);
            let search_end = std::cmp::min(line.len(), number.end() + 1);

            // Fetch search segments of the previous line, current line, and next line and concatenate them
            let mut search_string = String::new();
            if i > 0 {
                search_string.push_str(&lines[i - 1][search_start..search_end]);
            }
            search_string.push_str(&line[search_start..search_end]);
            if i < lines.len() - 1 {
                search_string.push_str(&lines[i + 1][search_start..search_end]);
            }

            // Search for symbols in the search string
            if symbol_regex.is_match(&search_string) {
                // If a symbol is found, it means the number is a part number
                part_numbers.push(number.as_str().parse::<u32>().unwrap());
            }
        }
    }

    part_numbers.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, "4361")
    }
}
