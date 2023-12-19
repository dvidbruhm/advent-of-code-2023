fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Box {
    lens: Vec<(String, usize)>,
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(",").collect::<Vec<&str>>()
}

fn hash(seq: &str) -> u32 {
    let hash: u32 = seq
        .chars()
        .filter(|c| *c != '\n')
        .filter(|c| c.is_alphabetic())
        .map(|c| c as u32)
        .fold(0, |acc, n| (acc + n) * 17 % 256);
    hash
}

fn part1(input: &str) -> String {
    let mut boxes: Vec<Box> = vec![Box { lens: vec![] }; 256];
    let sequences = parse_input(input);
    sequences.iter().for_each(|seq| {
        let box_nb = hash(seq) as usize;
        match seq.contains("=") {
            true => {
                let label = seq.split("=").take(1).collect();
                let index = boxes[box_nb].lens.iter().position(|l| l.0 == label);
                let focal = seq
                    .chars()
                    .filter(|c| *c != '\n')
                    .last()
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize;
                match index {
                    Some(ind) => {
                        boxes[box_nb].lens[ind] = (label, focal);
                    }
                    None => {
                        boxes[box_nb].lens.push((label, focal));
                    }
                }
            }
            false => {
                let label = seq.split("-").take(1).collect::<String>();
                let index = boxes[box_nb].lens.iter().position(|l| l.0 == label);
                match index {
                    Some(ind) => {
                        boxes[box_nb].lens.remove(ind);
                    }
                    None => {}
                }
            }
        }
    });

    let sum: usize = boxes
        .iter()
        .enumerate()
        .map(|(j, b)| {
            let s = b
                .lens
                .iter()
                .enumerate()
                .map(|(i, (_l, f))| (j + 1) * (i + 1) * f)
                .sum::<usize>();
            s
        })
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, "145".to_string());
    }
}
