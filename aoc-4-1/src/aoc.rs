use std::collections::HashSet;
use regex::Regex;

struct Game {
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

fn remove_empty_lines(lines: Vec<String>) -> Vec<String> {
    lines.into_iter().filter(|line| !line.trim().is_empty()).collect()
}

fn parse_number_string(number_string: &str) -> Vec<u32> {
    return number_string.split(" ").filter(|s| {
        return !s.is_empty();
    }).map(|s| {
        return s.parse().unwrap();
    }).collect();
}

fn parse_game(line: &String) -> Result<Game, String> {
    let pattern = "^Card[[:space:]]+[[:digit:]]+:((?:[[:space:]]+[[:digit:]]+)+) \\|((?:[[:space:]]+[[:digit:]]+)+)[[:space:]]*$";
    let game_regex = Regex::new(pattern).unwrap();
    match game_regex.captures(line) {
       Some(captures) => {
           let winning_numbers = parse_number_string(captures.get(1).unwrap().as_str());
           let card_numbers = parse_number_string(captures.get(2).unwrap().as_str());
           return Ok(Game{ winning_numbers: winning_numbers, card_numbers: card_numbers});
       },
       None => {
           return Err(format!("pattern didn't match: pattern={}", pattern));
       } 
    }
}

fn score_game(game: Game) -> u32 {
    let mut winning_numbers_set: HashSet<u32> = HashSet::new();
    for winning_number in game.winning_numbers {
        winning_numbers_set.insert(winning_number);
    }

    let mut card_numbers_set: HashSet<u32> = HashSet::new();
    for card_number in game.card_numbers {
        card_numbers_set.insert(card_number);
    }

    let overlapping_set = winning_numbers_set.intersection(&card_numbers_set);
    let match_count = overlapping_set.count();
    if match_count == 0 {
        return 0;
    }
    return 1 << (match_count - 1);
}

pub fn aoc_4_1(lines: Vec<String>) -> u32 {
    let fixed_lines = remove_empty_lines(lines);
    let mut score_sum: u32 = 0;
    for line in fixed_lines {
        match parse_game(&line) {
            Ok(game) => {
                score_sum += score_game(game);
            },
            Err(error) => {
                eprintln!("warning: line didn't game card pattern, skipping: line={} error={}", line, error);
                continue;
            }
        }
    }
    return score_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_4_1() {
        let test_data = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        let lines: Vec<String> = test_data.lines()
                                           .map(|line| line.to_string())
                                           .collect();
        let result = aoc_4_1(lines);
        assert_eq!(result, 13, "aoc-4-1 example did not return expected result");
    }
}
