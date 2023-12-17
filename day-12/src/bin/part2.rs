use itertools::Itertools;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<(String, Vec<i32>)> {
    let parsed: Vec<(String, Vec<i32>)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let nums: Vec<i32> = vec![parts[1]; 5]
                .join(",")
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let springs = vec![parts[0]; 5].join("?").clone();
            (springs, nums)
        })
        .collect();
    parsed
}

fn count_springs(line: &str) -> Vec<i32> {
    line.split(".")
        .filter(|s| s.len() > 0)
        .map(|s| s.len() as i32)
        .collect::<Vec<i32>>()
}

fn process_line(line: &str, nbs: &Vec<i32>) -> i32 {
    let missing_springs: i32 =
        nbs.iter().sum::<i32>() - line.chars().filter(|c| c == &'#').count() as i32;
    let unknown = line.chars().filter(|c| c == &'?').count();
    // let items = [vec![1; missing_springs as usize], vec![0; unknown]].concat();
    let mut count = 0;
    dbg!(line);
    for perm in (0..unknown).combinations(missing_springs as usize) {
        let mut cou = 0;
        let new_line: &str = &line
            .chars()
            .enumerate()
            .map(|(_i, c)| {
                if c == '?' {
                    if perm.contains(&cou) {
                        cou += 1;
                        return '#';
                    }
                    cou += 1;
                    return '.';
                }
                return c;
            })
            .collect::<String>();

        // dbg!(&new_line);
        if &count_springs(&new_line) == nbs {
            count += 1;
        }
        // dbg!(&perm);
    }
    // dbg!(count);
    count
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);
    let sum: i32 = parsed
        .iter()
        .map(|(line, nbs)| process_line(line, nbs))
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, "525152".to_string());
    }
}
