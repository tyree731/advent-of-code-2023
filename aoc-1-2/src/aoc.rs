use lazy_static::lazy_static;
use maplit::hashmap;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref DIGITS_MAP: HashMap<String, i32> = hashmap! {
        String::from("one") => 1,
        String::from("two") => 2,
        String::from("three") => 3,
        String::from("four") => 4,
        String::from("five") => 5,
        String::from("six") => 6,
        String::from("seven") => 7,
        String::from("eight") => 8,
        String::from("nine") => 9,
        String::from("1") => 1,
        String::from("2") => 2,
        String::from("3") => 3,
        String::from("4") => 4,
        String::from("5") => 5,
        String::from("6") => 6,
        String::from("7") => 7,
        String::from("8") => 8,
        String::from("9") => 9,
    };
}

pub fn aoc_1_2(lines: Vec<String>) -> i32 {
    if lines.is_empty() {
        return 0;
    }

    let forward_regex = Regex::new("([[:digit:]]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let backwards_regex = Regex::new("([[:digit:]]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let mut result = 0;
    for line in lines {
        if line.is_empty() {
            // empty line?, skip over it
            continue;
        }

        let mut first_digit: i32 = -1;
        match forward_regex.captures(&line) {
            Some(captures) => {
                if let Some(matched) = captures.get(1) {
                    first_digit = DIGITS_MAP[matched.as_str()];
                }
            },
            None => {
                // line continues no digit-like strings, skip over it
                continue;
            }
        }

        let line_reversed: String = line.chars().rev().collect();
        let backwards_captures = backwards_regex.captures(&line_reversed).unwrap();
        let mut second_digit: i32 = -1;
        if let Some(matched) = backwards_captures.get(1) {
            let reverse_match: String = matched.as_str().chars().rev().collect();
            second_digit = DIGITS_MAP[&reverse_match];
        }

        result = result + (first_digit * 10 + second_digit);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_1_2() {
        let test_data = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        "#;
        let lines: Vec<String> = test_data.lines()
                                           .map(|line| line.to_string())
                                           .collect();
        let result = aoc_1_2(lines);
        assert_eq!(result, 281, "aoc-1-2 example did not return expected result");
    }
}
