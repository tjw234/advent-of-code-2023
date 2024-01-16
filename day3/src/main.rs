use std::fs;
use std::env;
use std::cmp;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    println!("In file {}", file_path);
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let line_array: Vec<&str> = contents.lines().by_ref().collect();
    let lines_as_2d_array: Vec<Vec<char>> = line_array.iter().map(|x| x.chars().collect()).collect();
}
fn part1_find_possible_part_numbers(blueprint_lines: Vec<&str>) -> Vec<Vec<regex::Match>>{
    let find_number = Regex::new(r"[0-9]+").unwrap();
    blueprint_lines.iter().map(|line| find_number.find_iter(line).collect()).collect()
}
fn get_part_numbers_as_int(){

}
pub trait CharExt{
    fn is_not_digit_or_period(self) -> bool;
}
impl CharExt for char{
    fn is_not_digit_or_period(self) -> bool{
        !self.is_numeric() && self != '.'
    }
}
fn is_valid_part_number(possible_part : regex::Match, line_number: usize, lines_as_2d_array : Vec<Vec<char>>) -> bool {
    let maximum_line_index : usize = lines_as_2d_array.len()-1;
    let maximum_width_index : usize = lines_as_2d_array[0].len()-1;
    let line_range = line_number.saturating_sub(1)..cmp::min(maximum_line_index, line_number + 1);
    let symbol_search_range = possible_part.start().saturating_sub(1)..cmp::min(possible_part.end() + 1, maximum_width_index);
    let chars_to_search: Vec<&char> = lines_as_2d_array[line_range][symbol_search_range].into_iter().flatten().collect();
    chars_to_search.iter().any(|x| x.is_not_digit_or_period())
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
        let match_vec = part1_find_possible_part_numbers(line_array);
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
        let match_vec = part1_find_possible_part_numbers(line_array);
        assert!(is_valid_part_number())
    }
    
}