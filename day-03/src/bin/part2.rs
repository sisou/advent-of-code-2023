use regex::Regex;

fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

fn run(input: &str) -> String {
    let gear_regex = Regex::new("\\*").unwrap();
    let number_regex = Regex::new("[0-9]+").unwrap();

    let mut gear_ratios: Vec<u32> = vec![];

    let lines: Vec<_> = input.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let gears = gear_regex.find_iter(line);
        for gear in gears {
            // Exactly two adjacent numbers, even diagonally, mean that the asterix is a gear
            let search_start = gear.start().saturating_sub(1);
            let search_end = std::cmp::min(line.len(), gear.end() + 1);

            // (relative line number [-1, 0, 1], character position in that line)
            let mut found_number_parts: Vec<(isize, usize)> = vec![];

            // Search the previous line, current line, and next line and store found number parts
            if i > 0 {
                for number in number_regex.find_iter(&lines[i - 1][search_start..search_end]) {
                    found_number_parts.push((-1, number.start() + search_start));
                }
            }
            for number in number_regex.find_iter(&line[search_start..search_end]) {
                found_number_parts.push((0, number.start() + search_start));
            }
            if i < lines.len() - 1 {
                for number in number_regex.find_iter(&lines[i + 1][search_start..search_end]) {
                    found_number_parts.push((1, number.start() + search_start));
                }
            }

            // Skip gears that don't have exactly two adjacent numbers
            if found_number_parts.len() != 2 {
                continue;
            }

            // Find the full adjacent numbers
            let adjacent_numbers: Vec<u32> = found_number_parts
                .iter()
                .map(|(line_offset, char_offset)| {
                    let line = &lines[((i as isize) + *line_offset) as usize];
                    number_regex
                        .find_iter(line)
                        .filter_map(|number| {
                            if number.start() <= *char_offset && number.end() > *char_offset {
                                Some(number.as_str().parse::<u32>().unwrap())
                            } else {
                                None
                            }
                        })
                        .next()
                        .unwrap()
                })
                .collect();

            gear_ratios.push(adjacent_numbers[0] * adjacent_numbers[1]);
        }
    }

    gear_ratios.iter().sum::<u32>().to_string()
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
        assert_eq!(result, "467835")
    }
}
