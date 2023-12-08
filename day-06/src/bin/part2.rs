fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u64,
    dist: u64,
}

fn compute_dist(speed: u64, move_time: u64) -> u64 {
    speed * move_time
}

fn part1(input: &str) -> String {
    let raw: Vec<_> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter(|s| s.len() > 0)
                .filter(|s| s.parse::<u64>().is_ok())
                .fold("".to_owned(), |acc, s| acc + s)
                .parse::<u64>()
                .unwrap()
        })
        .collect();
    let race: Race = Race {
        time: raw[0],
        dist: raw[1],
    };

    let result = (0..race.time / 2)
        .map(|i| compute_dist(i as u64, race.time - i as u64))
        .filter(|d| d > &race.dist)
        .count()
        * 2
        + 1
        + (race.time as usize % 2);

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "71503".to_string());
    }
}
