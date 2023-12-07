fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let sections: Vec<_> = input.split("\n\n").collect();
    let seed_ranges: Vec<_> = sections[0]
        .split(": ")
        .skip(1)
        .flat_map(|s| {
            s.split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    println!("start");
    let mut seeds: Vec<u64> = vec![];
    for i in (0..seed_ranges.len()).step_by(2) {
        seeds.extend(seed_ranges[i]..seed_ranges[i] + seed_ranges[i + 1])
    }

    println!("end {}", seeds.len());

    let _ = sections[1..].iter().for_each(|section| {
        let rules: Vec<_> = section
            .lines()
            .skip(1)
            .map(|r| {
                r.split(" ")
                    .map(|s| {
                        let a = s.parse::<u64>().unwrap();
                        a
                    })
                    .collect::<Vec<u64>>()
            })
            .collect();
        seeds.iter_mut().enumerate().for_each(|(i, s)| {
            if i % 100000000 == 0 {
                println!("{}", i);
            }
            let mut new_seed: Option<u64> = None;
            for r in rules.iter() {
                if *s >= r[1] && *s < (r[1] + r[2]) {
                    new_seed = Some(*s - r[1] + r[0]);
                    break;
                }
            }
            // rules.iter().for_each(|r| {
            //     if *s >= r[1] && *s < (r[1] + r[2]) {
            //         new_seed = Some(*s - r[1] + r[0]);
            //     }
            // });
            match new_seed {
                None => new_seed = Some(*s),
                _ => {}
            }
            *s = new_seed.unwrap();
        });
    });

    seeds.iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, "46".to_string());
    }
}
