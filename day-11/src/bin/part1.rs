fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let (mut max_x, mut max_y) = (0, 0);
    let gals: Vec<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            max_y += 1;
            line.chars()
                .enumerate()
                .inspect(|_c| {
                    max_x += 1;
                })
                .filter(|(x, c)| *c == '#')
                .map(|(x, c)| (x as i32, y as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<(i32, i32)>>();

    max_y -= 1;
    dbg!(max_y, max_x / max_y);
    //let empty_rows = (0..gals.iter().map(|(x, y)| y)
    gals
}

fn part1(input: &str) -> String {
    let gals = parse_input(input);
    dbg!(gals);
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....",
        );
        assert_eq!(result, "374".to_string());
    }
}
