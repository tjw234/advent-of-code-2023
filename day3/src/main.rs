use std::fs;
use std::env;
use regex::Regex;
use core::ops::Range;

fn main() {
    let contents: String = read_file_to_string();
    let line_array: Vec<&str> = contents.lines().by_ref().collect();
    let lines_as_2d_array: Vec<Vec<char>> = line_array.iter().map(|x| x.chars().collect()).collect();
    run_part_1(line_array, lines_as_2d_array);
}

fn read_file_to_string() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    println!("Input file {}", file_path);
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn run_part_1(line_array: Vec<&str>, lines_as_2d_array: Vec<Vec<char>>){
    let match_vec = part1_find_possible_part_numbers(&line_array);
    let mut sum = 0;
    for line in 0..match_vec.len(){
        let part_sum: i32= match_vec[line].iter().filter(|part_num| is_valid_part_number(part_num, &line, &lines_as_2d_array)).map(|part_num| get_part_numbers_as_int(part_num)).sum();
        sum += part_sum;
    }
    println!("Part 1 Sum: {sum}");
}

fn part1_find_possible_part_numbers<'a>(blueprint_lines: &'a Vec<&str>) -> Vec<Vec<regex::Match<'a>>>{
    let find_number = Regex::new(r"[0-9]+").unwrap();
    blueprint_lines.iter().map(|line| find_number.find_iter(line).collect()).collect()
}

fn get_part_numbers_as_int(part_num: &regex::Match) -> i32{
    part_num.as_str().parse().unwrap()
}

fn calc_line_range(line_number: &usize, maximum_line_index: usize) -> Range<usize> {
    line_number.saturating_sub(1)..(line_number + 2).clamp(0, maximum_line_index)
}

fn calc_char_range(possible_part : &regex::Match, line_width: usize) -> Range<usize>{
    possible_part.start().saturating_sub(1)..(possible_part.end()+1).clamp(0, line_width)
}

fn is_valid_part_number(possible_part : &regex::Match, line_number: &usize, lines_as_2d_array : &Vec<Vec<char>>) -> bool {
    let line_range = calc_line_range(line_number, lines_as_2d_array.len());
    let symbol_search_range = calc_char_range(possible_part, lines_as_2d_array[0].len());
    let chars_to_search: Vec<char> = slice_2d_array(&lines_as_2d_array, &line_range, &symbol_search_range);
    chars_to_search.iter().any(|x| x.is_not_digit_or_period())
}

fn slice_2d_array (lines_as_2d_array: &Vec<Vec<char>>, line_range: &std::ops::Range<usize> , string_range: &std::ops::Range<usize> ) -> Vec<char> {
    lines_as_2d_array.iter()
        .skip(line_range.start)
        .take(line_range.len())
        .map(|s| s.iter()
                 .skip(string_range.start)
                 .take(string_range.len())).flatten().cloned().collect::<Vec<char>>()
}

/* This section extends the char type to include a new filter type. The reason for this is to make iter search more straightforeward
 */
pub trait CharExt{
    fn is_not_digit_or_period(self) -> bool;
}
impl CharExt for char{
    fn is_not_digit_or_period(self) -> bool{
        !self.is_numeric() && self != '.'
    }
}

fn part2_find_possible_gears<'a>(blueprint_lines: &'a Vec<&str>) -> Vec<Vec<regex::Match<'a>>>{
    let find_gear = Regex::new(r"\*").unwrap();
    blueprint_lines.iter().map(|line| find_gear.find_iter(line).collect()).collect()
}

fn is_valid_gear(possible_gear : &regex::Match, line_number: &usize, lines_as_2d_array : &Vec<Vec<char>>) -> bool {
    let line_range = calc_line_range(line_number, lines_as_2d_array.len());
    let symbol_search_range = calc_char_range(possible_gear, lines_as_2d_array[0].len());
    let chars_to_search: Vec<char> = slice_2d_array(&lines_as_2d_array, &line_range, &symbol_search_range);
    println!("{:?}", chars_to_search);
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn test_part1_find_possible_part_numbers() {
        let base_string = indoc!{
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."};
        let line_array: Vec<&str> = base_string.lines().by_ref().collect();
        let match_vec = part1_find_possible_part_numbers(&line_array);
        println!("{:?}", match_vec[0][1].range());
        assert!(match_vec[0][0].range() == (0..3));
        assert!(match_vec[0][1].range() == (5..8));
        assert!(match_vec[2][0].range() == (2..4));
        assert!(match_vec[2][1].range() == (6..9));
    }
    #[test]
    fn test_is_valid_part_number() {
        let base_string = indoc!{
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."};
        let line_array: Vec<&str> = base_string.lines().by_ref().collect();
        let lines_as_2d_array: Vec<Vec<char>> = line_array.iter().map(|x| x.chars().collect()).collect();
        let match_vec = part1_find_possible_part_numbers(&line_array);
        println!("{:?}", match_vec);

        assert!(    is_valid_part_number(&match_vec[0][0], &0, &lines_as_2d_array));
        assert!(!   is_valid_part_number(&match_vec[0][1], &0, &lines_as_2d_array));
        assert!(    is_valid_part_number(&match_vec[2][0], &2, &lines_as_2d_array));
        assert!(    is_valid_part_number(&match_vec[2][1], &2, &lines_as_2d_array));
        assert!(    is_valid_part_number(&match_vec[4][0], &4, &lines_as_2d_array));
        assert!(!   is_valid_part_number(&match_vec[5][0], &5, &lines_as_2d_array));
        assert!(    is_valid_part_number(&match_vec[6][0], &6, &lines_as_2d_array));
        assert!(    is_valid_part_number(&match_vec[7][0], &7, &lines_as_2d_array));
        assert!(    is_valid_part_number(&match_vec[9][0], &9, &lines_as_2d_array));
        assert!(    is_valid_part_number(&match_vec[9][1], &9, &lines_as_2d_array));
    }
    #[test]
    fn part1_integration_test() {
        let base_string = indoc!{
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."};
        let line_array: Vec<&str> = base_string.lines().by_ref().collect();
        let lines_as_2d_array: Vec<Vec<char>> = line_array.iter().map(|x| x.chars().collect()).collect();
        let match_vec = part1_find_possible_part_numbers(&line_array);
        let mut sum = 0;
        for line in 0..match_vec.len(){
            let part_sum: i32= match_vec[line].iter().filter(|part_num| is_valid_part_number(part_num, &line, &lines_as_2d_array)).map(|part_num| get_part_numbers_as_int(part_num)).sum();
            println!("Line {line} sum: {part_sum}");
            sum += part_sum;
        }
        assert_eq!(sum, 4361);
    }
    #[test]
    fn part1_integration_test_2() {
        let base_string = indoc!{
            "12.......*..
            +.........34
            .......-12..
            ..78........
            ..*....60...
            78..........
            .......23...
            ....90*12...
            ............
            2.2......12.
            .*.........*
            1.1.......56"};
        let line_array: Vec<&str> = base_string.lines().by_ref().collect();
        let lines_as_2d_array: Vec<Vec<char>> = line_array.iter().map(|x| x.chars().collect()).collect();
        let match_vec = part1_find_possible_part_numbers(&line_array);
        // assert!(    is_valid_part_number(&match_vec[11][2], &11, &lines_as_2d_array));
        let mut sum = 0;
        for line in 0..match_vec.len(){
            let part_sum: i32= match_vec[line].iter().filter(|part_num| is_valid_part_number(part_num, &line, &lines_as_2d_array)).map(|part_num| get_part_numbers_as_int(part_num)).sum();
            println!("Line {line} sum: {part_sum}");
            sum += part_sum;
        }
        assert_eq!(sum, 413);
    }
    #[test]
    fn test_find_possible_gears(){
        let base_string = indoc!{
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."};
        let line_array: Vec<&str> = base_string.lines().by_ref().collect();
        let match_vec = part2_find_possible_gears(&line_array);
        println!("{:?}", match_vec[1][0].range());
        assert!(match_vec[1][0].range() == (3..4));
        assert!(match_vec[4][0].range() == (3..4));
        assert!(match_vec[8][0].range() == (5..6));
    }
    #[test]
    fn test_is_valid_gear(){
        let base_string = indoc!{
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."};
        let line_array: Vec<&str> = base_string.lines().by_ref().collect();
        let match_vec = part2_find_possible_gears(&line_array);

        let lines_as_2d_array: Vec<Vec<char>> = line_array.iter().map(|x| x.chars().collect()).collect();
        assert!(is_valid_gear(&match_vec[1][0], &1, &lines_as_2d_array));
        assert!(! is_valid_gear(&match_vec[4][0], &4, &lines_as_2d_array));
        assert!(is_valid_gear(&match_vec[8][0], &8, &lines_as_2d_array));
    }
}