fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

struct Sequence {
    values: Vec<i32>,
}

impl Sequence {
    pub fn next_value(&self) -> i32 {
        if self.values.iter().all(|v| *v == 0) {
            return 0;
        }

        let mut derived_sequence = Vec::with_capacity(self.values.len() - 1);

        for (i, val) in self.values.iter().enumerate() {
            if i == self.values.len() - 1 {
                break;
            }
            derived_sequence.push(self.values[i + 1] - val);
        }

        let next_sequence = Sequence {
            values: derived_sequence,
        };

        self.values.last().unwrap() + next_sequence.next_value()
    }
}

fn run(input: &str) -> String {
    let lines = input.lines();

    let sequences = lines.map(|line| {
        let values: Vec<_> = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        Sequence { values }
    });

    let next_values = sequences.map(|s| s.next_value());

    next_values.sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");
        assert_eq!(result, "114")
    }
}
