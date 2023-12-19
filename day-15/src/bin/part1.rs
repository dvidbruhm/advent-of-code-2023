fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(",").collect::<Vec<&str>>()
}

fn hash(seq: &str) -> u32 {
    let hash: u32 = seq
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c as u32)
        .fold(0, |acc, n| (acc + n) * 17 % 256);
    hash
}

fn part1(input: &str) -> String {
    let sequences = parse_input(input);
    let sum: u32 = sequences.iter().map(|seq| hash(seq)).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, "1320".to_string());
    }
}
