fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Map {
    pub fn from_line(line: &str) -> Self {
        let mut parts = line.split(' ');
        Self {
            destination_range_start: parts.next().unwrap().parse::<u64>().unwrap(),
            source_range_start: parts.next().unwrap().parse::<u64>().unwrap(),
            range_length: parts.next().unwrap().parse::<u64>().unwrap(),
        }
    }

    pub fn map(&self, value: u64) -> Option<u64> {
        if value >= self.source_range_start && value < self.source_range_start + self.range_length {
            Some(value - self.source_range_start + self.destination_range_start)
        } else {
            None
        }
    }
}

fn run(input: &str) -> String {
    let mut next_values: Vec<u64> = vec![];
    let mut current_values: Vec<u64> = vec![];

    let lines: Vec<_> = input.lines().collect();

    for line in lines.iter() {
        if line.is_empty() {
            // We've reached the end of a section
            // Move all next values to current values and leave remaining current values as-is
            current_values.append(&mut next_values);
            continue;
        }

        if line.starts_with("seeds:") {
            line.split(": ").nth(1).unwrap().split(' ').for_each(|x| {
                next_values.push(x.parse::<u64>().unwrap());
            });
            continue;
        }

        if line.contains(':') {
            println!("Processing {}", line);
            continue; // Skip header lines
        }

        let map = Map::from_line(line);

        current_values = current_values
            .iter()
            .filter_map(|v| {
                if let Some(next_value) = map.map(*v) {
                    // If we can map the current value to a next value, add it to the next values
                    next_values.push(next_value);
                    // And remove from current values
                    None
                } else {
                    // If we cannot map it, keep it in current values
                    Some(*v)
                }
            })
            .collect::<Vec<_>>();
    }

    // Move all values into one array to find the lowest value
    current_values.append(&mut next_values);
    current_values.sort_unstable();
    current_values[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        assert_eq!(result, "35");
    }
}
