fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for (_, line) in input.to_string().split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        let game_part = line.split(":").next().unwrap();
        let game_nb: i32 = game_part
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let records_part = line.split(":").skip(1).next().unwrap();
        let subsets: Vec<_> = records_part.split([';', ',']).map(|a| a.trim()).collect();
        let mut skip = false;
        for s in subsets.iter() {
            let a: Vec<_> = s.split(' ').collect();
            let n: i32 = a[0].parse().unwrap();
            match a[1] {
                "green" => {
                    if n > 13 {
                        skip = true
                    }
                }
                "red" => {
                    if n > 12 {
                        skip = true
                    }
                }
                "blue" => {
                    if n > 14 {
                        skip = true
                    }
                }
                _ => {}
            }
        }
        if skip == false {
            sum += game_nb
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
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8".to_string());
    }
}
