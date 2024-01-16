// extern crate regex;
use std::char;
use std::fs;
use std::env;

use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    println!("In file {}", file_path);
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let part_1_sum: u32 = contents.lines().by_ref().map(|line: &str| part1_translate_line(line)).sum();
    println!("Final config is {} for part 1", part_1_sum);
    let part_2_sum: u32 = contents.lines().by_ref().map(|line: &str| translate_line(line)).sum();
    println!("Final config is {} for part 2", part_2_sum);
}
fn part1_parse_first(line:&str) -> u32 {
    line.chars().find(|c| c.is_digit(10)).and_then(|c| c.to_digit(10)).unwrap()
}
fn part1_parse_last(line:&str) -> u32 {
    line.chars().rev().find(|c| c.is_digit(10)).and_then(|c| c.to_digit(10)).unwrap()
}
fn part1_translate_line(line:&str) -> u32 {
    part1_parse_first(line) * 10 + part1_parse_last(line)
}
fn parse_first(line:&str) -> u32 {
    let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_match: regex::Match = re.find(line).unwrap();
    match re_match.len() {
        1 => re_match.as_str().chars().find(|c| c.is_digit(10)).and_then(|c| c.to_digit(10)).unwrap(),
        _ => convert_number_name_to_u32(re_match.as_str()).unwrap()
    }
}
fn convert_number_name_to_u32(number: &str) -> Option<u32> {
    match number {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
    }
}
fn parse_last(line:&str) -> u32 {
    let rev_line = line.chars().rev().collect::<String>();
    let re = Regex::new(r"([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let re_match: regex::Match = re.find(rev_line.as_ref()).unwrap();
    let rev_re_match: String = re_match.as_str().chars().rev().collect();
    match re_match.len() {
        1 => rev_re_match.as_str().chars().find(|c| c.is_digit(10)).and_then(|c| c.to_digit(10)).unwrap(),
        _ => convert_number_name_to_u32(rev_re_match.as_str()).unwrap()
    }
}
fn translate_line(line: &str) -> u32 {
    parse_first(line) * 10 + parse_last(line)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_first() {
        assert_eq!(parse_first("two1nine"), 2);
        assert_eq!(parse_first("eightwothree"), 8);
        assert_eq!(parse_first("abcone2threexyz"), 1);
        assert_eq!(parse_first("xtwone3four"), 2);
        assert_eq!(parse_first("4nineeightseven2"), 4);
        assert_eq!(parse_first("zoneight234"), 1);
        assert_eq!(parse_first("7pqrstsixteen"), 7);
    }
    #[test]
    fn test_parse_last() {
        assert_eq!(parse_last("two1nine"), 9);
        assert_eq!(parse_last("eightwothree"), 3);
        assert_eq!(parse_last("abcone2threexyz"), 3);
        assert_eq!(parse_last("xtwone3four"), 4);
        assert_eq!(parse_last("4nineeightseven2"), 2);
        assert_eq!(parse_last("zoneight234"), 4);
        assert_eq!(parse_last("7pqrstsixteen"), 6);
    }
    #[test]
    fn test_translate_line() {
        assert_eq!(translate_line("two1nine"), 29);
        assert_eq!(translate_line("eightwothree"), 83);
        assert_eq!(translate_line("abcone2threexyz"), 13);
        assert_eq!(translate_line("xtwone3four"), 24);
        assert_eq!(translate_line("4nineeightseven2"), 42);
        assert_eq!(translate_line("zoneight234"), 14);
        assert_eq!(translate_line("7pqrstsixteen"), 76);
    }

}