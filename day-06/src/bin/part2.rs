fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u32,
    dist: u32,
}

fn compute_dist(speed: u32, move_time: u32) -> u32 {
    speed * move_time
}

fn part1(input: &str) -> String {
    let raw: Vec<_> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter(|s| s.len() > 0)
                .filter(|s| s.parse::<u32>().is_ok())
                .fold("".to_owned(), |acc, s| acc + s)
                .parse::<u32>()
        })
        .collect();
    dbg!(&raw);
    let mut race: Race = Race { time: raw[0][0], dist: raw[0][1] }

    let result = races
        .iter()
        .map(|r| {
            dbg!(r.time as usize % 2);
            (0..r.time / 2)
                .map(|i| compute_dist(i as u32, r.time - i as u32))
                .filter(|d| d > &r.dist)
                .count()
                * 2
                + 1
                + (r.time as usize % 2)
        })
        .inspect(|a| {
            dbg!(a);
        })
        .reduce(|acc, n| acc * n)
        .unwrap();

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
        assert_eq!(result, "288".to_string());
    }
}
