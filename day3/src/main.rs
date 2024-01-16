use std::fs;
use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    println!("In file {}", file_path);
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let line_array: Vec<&str> = contents.lines().by_ref().collect();
    let lines_as_2d_array: Vec<Vec<char>> = line_array.iter().map(|x| x.chars().collect()).collect();
}
fn part1_find_possible_part_numbers(blueprint: &str) -> Vec<regex::Match>{
    let find_number = Regex::new(r"[0-9]+").unwrap();
    find_number.find_iter(blueprint).collect()
}
fn get_part_numbers_as_int(){

}
fn is_valid_part_number(regex::Match possible_part, Vec<Vec<char>> lines_as_2d_array) -> bool {

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_find_possible_part_numbers() {
        let base_string ="467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
    let match_vec = part1_find_possible_part_numbers(base_string);
    println!("{:?}", match_vec[1].range());
    assert!(match_vec[0].range() == (0..3));
    assert!(match_vec[1].range() == (5..8));
    assert!(match_vec[2].range() == (40..42));
    assert!(match_vec[3].range() == (44..47));
    }
    #[test]
    fn test_part1_find_possible_part_numbers() {
        let base_string ="467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let line_array: Vec<&str> = base_string.lines().by_ref().collect();
        let lines_as_2d_array: Vec<Vec<char>> = line_array.iter().map(|x| x.chars().collect()).collect();

    }
    
}