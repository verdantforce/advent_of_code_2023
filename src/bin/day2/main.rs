// https://adventofcode.com/2023/day/1
// cargo run --bin day2
use std::convert::From;

#[derive(Default, Debug)]
struct Game {
    id: u32,
    sets: Vec<RGB>,
}

#[derive(Default, Debug, Clone, Copy)]
struct RGB {
    red: u32,
    blue: u32,
    green: u32,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (game_info, set_info) = line.split_once(": ").unwrap();
        let id = game_info.split(" ").nth(1).unwrap().parse().unwrap();
        let sets = set_info
            .split("; ")
            .map(|s| {
                let mut rgb: RGB = Default::default();
                for ss in s.split(", ") {
                    let (n, color) = ss.split_once(" ").unwrap();
                    let i: u32 = n.parse().unwrap();
                    match color {
                        "red" => rgb.red = i,
                        "blue" => rgb.blue = i,
                        "green" => rgb.green = i,
                        _ => unreachable!(),
                    }
                }
                rgb
            })
            .collect();

        Game { id, sets }
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let game = Game::from(line);
            if game
                .sets
                .iter()
                .any(|rgb| rgb.red > 12 || rgb.green > 13 || rgb.blue > 14)
            {
                0
            } else {
                game.id
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let game = Game::from(line);
            let mut iter = game.sets.iter();
            let mut min_rbg: RGB = iter.next().unwrap().clone();

            while let Some(rbg) = iter.next() {
                if min_rbg.red < rbg.red {
                    min_rbg.red = rbg.red
                }
                if min_rbg.blue < rbg.blue {
                    min_rbg.blue = rbg.blue
                }
                if min_rbg.green < rbg.green {
                    min_rbg.green = rbg.green
                }
            }

            // println!("line = {}, min_rbg = {:?}", line, min_rbg);

            min_rbg.red * min_rbg.blue * min_rbg.green
        })
        .sum()
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
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input_test.txt");
        assert_eq!(part2(input), 2286);
    }
}
