fn main() {
    let input = include_str!("../input1.txt");
    let result = run(input);
    dbg!(result);
}

struct Race {
    time: u32,
    distance_record: u32,
}

impl Race {
    fn distance_by_button_push_duration(&self, duration: u32) -> u32 {
        let speed = duration; // millimeter / millisecond
        let remaining_time = self.time - duration;
        speed * remaining_time
    }
}

fn run(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();

    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap());

    let distance_records: Vec<_> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut races = vec![];

    for (i, time) in times.enumerate() {
        races.push(Race {
            time,
            distance_record: distance_records[i],
        });
    }

    let mut ways_to_beat_the_record_by_game = vec![];

    for race in races {
        let mut ways_to_beat_the_record = 0usize;

        let mut push_duration = race.time / 2;

        while push_duration > 0 {
            let distance = race.distance_by_button_push_duration(push_duration);
            if distance <= race.distance_record {
                break;
            }
            ways_to_beat_the_record += 1;
            push_duration -= 1;
        }

        // Double the count to account for both sides of the half-time
        ways_to_beat_the_record *= 2;

        // Reduce by one when the time is even, as there is only one half-time
        if race.time % 2 == 0 {
            ways_to_beat_the_record -= 1;
        }

        ways_to_beat_the_record_by_game.push(ways_to_beat_the_record);
    }

    ways_to_beat_the_record_by_game
        .iter()
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, "288")
    }
}
