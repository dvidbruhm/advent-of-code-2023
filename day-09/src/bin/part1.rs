fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn predict(history: &Vec<i32>) -> i32 {
    let mut diffs: Vec<i32> = history.to_vec();
    let mut sum = 0;
    while !diffs.iter().all(|d| d == &0) {
        let mut next_diffs: Vec<i32> = vec![];
        for i in 0..diffs.len() - 1 {
            next_diffs.push(diffs[i + 1] - diffs[i]);
        }
        sum += diffs.last().unwrap();
        diffs = next_diffs;
    }
    sum
}

fn part1(input: &str) -> String {
    let histories = parse_input(input);
    let sum: i32 = histories.iter().map(|h| predict(h)).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, "114".to_string());
    }
}
