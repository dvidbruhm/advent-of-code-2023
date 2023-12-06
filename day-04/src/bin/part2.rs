use std::cmp;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let mut nb_cards: Vec<i32> = vec![1; lines.len()];
    let _: Vec<_> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let splits: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            let winning: Vec<i32> = splits[0]
                .split(':')
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .filter(|s| s.to_string() != "".to_string())
                .map(|n| n.trim().parse().unwrap())
                .collect();
            let nums: Vec<i32> = splits[1]
                .split(' ')
                .filter(|s| s.to_string() != "".to_string())
                .map(|n| n.trim().parse().unwrap())
                .collect();

            let sum: i32 = nums
                .iter()
                .map(|n| {
                    if winning.contains(n) {
                        return 1;
                    }
                    0
                })
                .sum();
            let current_n = nb_cards[i];
            if i + 1 <= nb_cards.len() - 1 {
                nb_cards[i + 1..cmp::min(i + 1 + sum as usize, lines.len())]
                    .iter_mut()
                    .for_each(|n| *n += current_n);
            }
        })
        .collect();

    let result: i32 = nb_cards.iter().sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30".to_string());
    }
}
