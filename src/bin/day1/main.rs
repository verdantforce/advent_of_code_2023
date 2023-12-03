// https://adventofcode.com/2023/day/1
// cargo run --bin day1

static DIGITS: phf::Map<&'static str, u32> = phf::phf_map! {
    "one" => 1,
    "two"=>2,
    "three"=>3,
    "four"=>4,
    "five"=>5,
    "six"=>6,
    "seven"=>7,
    "eight"=>8,
    "nine"=>9,
};

fn vec_to_n(digits: &Vec<u32>) -> u32 {
    digits
        .first()
        .zip(digits.last())
        .map(|(x, y)| x * 10 + y)
        .unwrap()
}

fn parse_line2(line: &str) -> Vec<u32> {
    (0..line.len())
        .filter_map(|i| {
            let substr = &line[i..];
            let digit = substr.chars().next().and_then(|c| c.to_digit(10));
            if digit.is_some() {
                return digit;
            } else {
                for (word, digit) in &DIGITS {
                    if substr.starts_with(word) {
                        return Some(*digit);
                    }
                }
                return None;
            }
        })
        .collect()
}

fn parse_line1(line: &str) -> Vec<u32> {
    line.chars().filter_map(|x| x.to_digit(10)).collect()
}

fn part1(input: &str) {
    let answer: u32 = input.lines().map(|line| vec_to_n(&parse_line1(line))).sum();
    println!("part1: {}", answer);
}

fn part2(input: &str) {
    let answer: u32 = input.lines().map(|line| vec_to_n(&parse_line2(line))).sum();
    println!("part2: {}", answer);
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line1() {
        assert_eq!(parse_line1("two1nine"), vec![1]);
        assert_eq!(parse_line1("abcone2threexyz"), vec![2]);
        assert_eq!(parse_line1("xtwone3four"), vec![3]);
        assert_eq!(parse_line1("4nineeightseven2"), vec![4, 2]);
        assert_eq!(parse_line1("zoneight234"), vec![2, 3, 4]);
        assert_eq!(parse_line1("7pqrstsixteen"), vec![7]);
    }

    #[test]
    fn test_parse_line2() {
        assert_eq!(parse_line2("two1nine"), vec![2, 1, 9]);
        assert_eq!(parse_line2("abcone2threexyz"), vec![1, 2, 3]);
        assert_eq!(parse_line2("xtwone3four"), vec![2, 1, 3, 4]);
        assert_eq!(parse_line2("4nineeightseven2"), vec![4, 9, 8, 7, 2]);
        assert_eq!(parse_line2("zoneight234"), vec![1, 8, 2, 3, 4]);
        assert_eq!(parse_line2("7pqrstsixteen"), vec![7, 6]);
    }
}
