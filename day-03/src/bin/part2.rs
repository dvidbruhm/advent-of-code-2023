use std::cmp;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut nums: Vec<(i32, i32, i32, i32)> = vec![];
    let mut spec: Vec<(i32, i32, char)> = vec![];

    let (mut x, mut y) = (0, 0);
    let lines: Vec<_> = input.lines().collect();
    let max_x = lines[0].len() - 1;
    let max_y = lines.len() - 1;
    let mut real_x = 0;

    let _: Vec<_> = lines
        .iter()
        .map(|line| {
            let mut s = "".to_string();
            for c in line.chars() {
                match c {
                    '0'..='9' => {
                        s.push(c);
                    }
                    _ => {
                        if s.is_empty() == false {
                            nums.push((x, y, s.len() as i32, s.parse().unwrap()));
                            x += s.len() as i32;
                            s = "".to_string();
                        }
                        if c == '*' {
                            spec.push((x, y, c));
                        }
                        x += 1;
                    }
                }
                if s.is_empty() == false && real_x == max_x {
                    nums.push((x, y, s.len() as i32, s.parse().unwrap()));
                }
                real_x += 1;
            }
            y += 1;
            x = 0;
            real_x = 0;
        })
        .collect();

    let mut sum = 0;
    for s in spec.iter() {
        let mut count = 0;
        let mut included: Vec<i32> = vec![];
        for n in nums.iter() {
            let valid_x = s.0 <= cmp::min(n.0 + n.2, max_x as i32) && s.0 >= cmp::max(0, n.0 - 1);
            let valid_y = s.1 <= cmp::min(n.1 + 1, max_y as i32) && s.1 >= cmp::max(0, n.1 - 1);
            if valid_x && valid_y {
                count += 1;
                included.push(n.3);
            }
        }
        if count == 2 {
            sum += included[0] * included[1];
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "467..114..
.1.*......
..35..633*
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "467835".to_string());
    }
}
