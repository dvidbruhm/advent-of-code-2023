fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let num_map = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let mut sum = 0;
    for (_, line) in input.to_string().split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut line2 = line.to_string();
        for (l, n) in num_map.iter() {
            line2 = line2.replace(l, n);
        }

        let nums: Vec<_> = line2.chars().filter_map(|c| c.to_digit(10)).collect();
        let digit: i32 = (nums.first().unwrap().to_string() + &nums.last().unwrap().to_string())
            .parse()
            .unwrap();
        sum += digit;
        // dbg!(i, line, &line2, nums, digit);
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142".to_string());
    }
}
