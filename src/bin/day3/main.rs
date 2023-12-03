// https://adventofcode.com/2023/day/3
// cargo run --bin day3

fn part1(input: &str) -> u32 {
    !unimplemented!()
}

fn part2(input: &str) -> u32 {
    !unimplemented!()
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
