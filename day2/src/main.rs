use std::fs;
use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    println!("In file {}", file_path);
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let limit: CubeCount = CubeCount{red :12, green :13, blue: 14};
    let part_1_sum: u32 = contents.lines().by_ref().map(|line: &str| game_valid(line, &limit)).sum();
    println!("Final sum is {} for part 1", part_1_sum);
    let part_2_sum: u32 = contents.lines().by_ref().map(|line: &str| get_game_power(line)).sum();
    println!("Final sum is {} for part 2", part_2_sum);
}
fn game_valid(game: &str, limit: &CubeCount) -> u32 {
    let re = Regex::new(r"(?:^Game )([0-9]+)(?::)").unwrap();
    if check_if_valid_game(game, limit){
        // println!("{}: {}", re.captures(game).unwrap().get(1).unwrap().as_str(), re.captures(game).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap());
        return re.captures(game).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
    }
    0
}
#[derive(Debug)]
struct CubeCount{
    red : u32,
    green : u32,
    blue : u32,
}
fn check_if_valid_counts(count: &CubeCount, limit: &CubeCount) -> bool {
    count.red <= limit.red && count.blue <= limit.blue && count.green <= limit.green
}
fn check_if_valid_game(game: &str, limit: &CubeCount) -> bool {
    let re = Regex::new(r"((?:(?:\s)+[0-9]+(?:\s)+(?:red|blue|green)(?:,)?)+[;]*)").unwrap();
    let pull_re = Regex::new(r"(?:([0-9]+)(?: )(r|g|b)(?:ed|reen|lue))+").unwrap();
    let mut max: CubeCount = CubeCount{red :0, blue :0, green :0};
    for (_, [pull]) in re.captures_iter(game).map(|c| c.extract()){
        let mut count: CubeCount = CubeCount{red :0, blue :0, green :0};
        for (_, [number, color]) in pull_re.captures_iter(pull).map(|c| c.extract()){
            match color {
                "r" => count.red = number.parse().unwrap(),
                "g" => count.green = number.parse().unwrap(),
                "b" => count.blue = number.parse().unwrap(),
                _ => panic!("Color wasn't red green or blue"),
            }
        }

        if !check_if_valid_counts(&count, limit) {
            return false;
        }
    }
    return true;
}
fn get_game_power(game: &str) -> u32 {
    let re = Regex::new(r"((?:(?:\s)+[0-9]+(?:\s)+(?:red|blue|green)(?:,)?)+[;]*)").unwrap();
    let pull_re = Regex::new(r"(?:([0-9]+)(?: )(r|g|b)(?:ed|reen|lue))+").unwrap();
    let mut max: CubeCount = CubeCount{red :0, blue :0, green :0};
    for (_, [pull]) in re.captures_iter(game).map(|c| c.extract()){
        let mut count: CubeCount = CubeCount{red :0, blue :0, green :0};
        for (_, [number, color]) in pull_re.captures_iter(pull).map(|c| c.extract()){
            match color {
                "r" => count.red = number.parse().unwrap(),
                "g" => count.green = number.parse().unwrap(),
                "b" => count.blue = number.parse().unwrap(),
                _ => panic!("Color wasn't red green or blue"),
            }
        }
        if count.red > max.red {max.red = count.red;}
        if count.green > max.green {max.green = count.green;}
        if count.blue > max.blue {max.blue = count.blue;}
    }
    return max.red * max.blue * max.green;
}



mod tests {
    use super::*;
    #[test]
    fn test_check_if_valid_counts() {
        let limit: CubeCount = CubeCount{red :12, blue :13, green :14};
        let count = CubeCount{red :0, blue :4, green :5};
        assert!(check_if_valid_counts(&count, &limit));
        let count = CubeCount{red :20, blue :4, green :5};
        assert!(!check_if_valid_counts(&count, &limit));
        let count = CubeCount{red :12, blue :13, green :14};
        assert!(check_if_valid_counts(&count, &limit));
    }
    #[test]
    fn test_check_if_valid_game() {
        let game: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let limit = CubeCount{red :12, blue :13, green :14};
        assert!(check_if_valid_game(game, &limit));
        let game: &str = "Game 1: 13 blue, 12 red; 1 red, 2 green, 6 blue; 14 green";
        assert!(check_if_valid_game(game, &limit));
    }
    #[test]
    fn test_check_if_valid_game_invalid() {
        let limit = CubeCount{red :12, blue :13, green :14};
        let game: &str = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert!(!check_if_valid_game(game, &limit));
    }    
    #[test]
    fn test_check_if_valid_game_invalid_2() {
        let limit = CubeCount{red :12, blue :13, green :14};
        let game: &str = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert!(!check_if_valid_game(game, &limit));
    }
}