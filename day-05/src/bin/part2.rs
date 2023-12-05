fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start_inclusive: u64,
    end_exclusive: u64,
}

#[derive(Debug)]
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

    /// Map a range to an included range and a list of split-off ranges
    pub fn map(&self, range: &Range) -> Option<(Range, Vec<Range>)> {
        // Set up inclusive ranges
        // x is the incoming range
        let x1 = range.start_inclusive;
        let x2 = range.end_exclusive - 1;
        // y is the checked range
        let y1 = self.source_range_start;
        let y2 = self.source_range_start + self.range_length - 1;

        if x2 < y1 || x1 > y2 {
            return None; // No overlap
        }

        // Calculate the overlap
        let start_common = if x1 < y1 { y1 } else { x1 };
        let end_common = if x2 < y2 { x2 } else { y2 };

        let mapped_range = Range {
            start_inclusive: start_common - self.source_range_start + self.destination_range_start,
            end_exclusive: end_common - self.source_range_start + self.destination_range_start + 1,
        };

        let mut split_off_ranges: Vec<Range> = vec![];
        // Add a potential unmapped range before the mapped range
        if x1 < y1 {
            split_off_ranges.push(Range {
                start_inclusive: x1,
                end_exclusive: y1,
            });
        }
        // Add a potential unmapped range after the mapped range
        if x2 > y2 {
            split_off_ranges.push(Range {
                start_inclusive: y2 + 1,
                end_exclusive: x2 + 1,
            });
        }

        Some((mapped_range, split_off_ranges))
    }
}

fn run(input: &str) -> String {
    let mut next_ranges: Vec<Range> = vec![];
    let mut current_ranges: Vec<Range> = vec![];

    let lines: Vec<_> = input.lines().collect();

    for line in lines.iter() {
        if line.is_empty() {
            // We've reached the end of a section
            // Move all next values to current values and leave remaining current values as-is
            current_ranges.append(&mut next_ranges);
            continue;
        }

        if line.starts_with("seeds:") {
            let mut start = 0u64;
            let mut length = 0u64;
            for (i, number) in line.split(": ").nth(1).unwrap().split(' ').enumerate() {
                if i % 2 == 0 {
                    start = number.parse::<u64>().unwrap();
                } else {
                    length = number.parse::<u64>().unwrap();
                }

                if i % 2 == 1 {
                    next_ranges.push(Range {
                        start_inclusive: start,
                        end_exclusive: start + length,
                    });
                }
            }
            continue;
        }

        if line.contains(':') {
            println!("Processing {}", line);
            continue; // Skip header lines
        }

        let map = Map::from_line(line);

        // We need to iterate over the current ranges while adding new ranges to the end of the list
        let mut split_off_ranges: Vec<Range> = vec![];

        current_ranges = current_ranges
            .iter()
            .filter_map(|r| {
                if let Some((mapped_range, unmapped_ranges)) = map.map(r) {
                    // If a range was mapped, add it to the next ranges
                    next_ranges.push(mapped_range);
                    // Add the unmapped ranges to the split-off ranges
                    split_off_ranges.extend(unmapped_ranges);
                    // And remove from current ranges
                    None
                } else {
                    // If the range was not mapped, keep it in the current ranges
                    Some(*r)
                }
            })
            .collect::<Vec<_>>();

        // Add all split-off ranges to the current ranges for the next mapping turn
        current_ranges.extend(split_off_ranges);
    }

    // Move all values into one array to find the lowest value
    current_ranges.append(&mut next_ranges);
    current_ranges.sort_unstable_by_key(|f| f.start_inclusive);
    current_ranges[0].start_inclusive.to_string()
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
        assert_eq!(result, "46");
    }
}
