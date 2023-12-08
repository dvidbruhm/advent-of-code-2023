use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn transform_cards(cards: &String) -> String {
    cards
        .replace("A", "F")
        .replace("K", "E")
        .replace("Q", "D")
        .replace("J", "0")
        .replace("T", "B")
}
fn transform_cards_back(cards: &String) -> String {
    cards
        .replace("F", "A")
        .replace("E", "K")
        .replace("D", "Q")
        .replace("0", "J")
        .replace("B", "T")
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    Pairs,
    Pair,
    High,
    None,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32,
    hand_type: HandType,
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let mut num_j = 0;
        let mut card_counts: Vec<i32> = self
            .cards
            .chars()
            .fold(HashMap::new(), |mut map, c| {
                match c {
                    '0' => num_j += 1,
                    _ => *map.entry(c).or_insert(0) += 1,
                }
                map
            })
            .values()
            .cloned()
            .collect();
        card_counts.push(0);
        card_counts.sort();
        let last = card_counts.last_mut().unwrap();
        *last += num_j;

        card_counts.retain(|&count| count > 1);

        match card_counts[..] {
            [5] => HandType::Five,
            [4] => HandType::Four,
            [2, 3] => HandType::Full,
            [3] => HandType::Three,
            [2, 2] => HandType::Pairs,
            [2] => HandType::Pair,
            _ => HandType::High,
        }
    }
}

fn part1(input: &str) -> String {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let l: Vec<&str> = line.split(" ").collect();
            Hand {
                cards: transform_cards(&l[0].to_string()),
                bid: l[1].parse::<i32>().unwrap(),
                hand_type: HandType::None,
            }
        })
        .map(|mut hand| {
            hand.hand_type = hand.get_hand_type();
            hand
        })
        .collect();

    hands.sort_by(|a, b| a.cards.cmp(&b.cards));
    hands.reverse();
    hands.sort_by_key(|hand| hand.hand_type);
    hands
        .iter_mut()
        .for_each(|hand| hand.cards = transform_cards_back(&hand.cards));

    let sum: usize = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "5905".to_string());
    }
}
