// https://adventofcode.com/2023/day/3
// cargo run --bin day4

#[allow(dead_code)]
#[derive(Debug)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    owned: Vec<u32>,
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let (card, numbers) = line.split_once(":").unwrap();
        let id = card.split_whitespace().nth(1).unwrap().parse().unwrap();
        let (w, h) = numbers.split_once(" | ").unwrap();
        let winners = w
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let owned = h
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        Card { id, winners, owned }
    }
}

impl Card {
    fn score(&self) -> u32 {
        let m = self.num_matches();
        if m == 0 {
            0
        } else {
            1 << (m - 1)
        }
    }

    fn num_matches(&self) -> u32 {
        self.owned
            .iter()
            .filter(|&n| self.winners.contains(n))
            .count() as u32
    }
}

fn part1(input: &str) -> u32 {
    input.lines().map(|line| Card::from(line).score()).sum()
}

fn part2(input: &str) -> u32 {
    let num_matches: Vec<u32> = input
        .lines()
        .map(|line| Card::from(line).num_matches())
        .collect();

    let mut points = vec![1; num_matches.len()];

    for i in 0..points.len() {
        for j in 1..=num_matches[i] {
            points[i + j as usize] += points[i];
        }
    }
    points.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("input_test.txt");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_card_from() {
        assert_eq!(
            Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").score(),
            8
        );
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        assert_eq!(part2(input), 30);
    }
}
