fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for (i, line) in input.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        let nums: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        let digit: i32 = (nums.first().unwrap().to_string() + &nums.last().unwrap().to_string())
            .parse()
            .unwrap();
        sum += digit;
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
