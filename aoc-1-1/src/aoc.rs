fn str_index_to_i32(str: &str, index: usize) -> i32 {
    return str.chars().nth(index).unwrap() as i32 - '0' as i32;
}

pub fn aoc_1_1(lines: Vec<String>) -> i32 {
    if lines.is_empty() {
        return 0;
    }

    let mut result = 0;
    for line in lines {
        if line.is_empty() {
            // empty line?, skip over it
            continue;
        }

        let first_digit_index = line.chars().position(|c| c.is_ascii_digit());
        if first_digit_index.is_none() {
            // line continues no digits, skip over it
            continue;
        }

        let second_digit_index = line.char_indices().rev().find_map(|(index, c)| {
            if c.is_ascii_digit() {
                Some(index)
            } else {
                None
            }
        });

        let first_digit = str_index_to_i32(&line, first_digit_index.unwrap());
        let second_digit = str_index_to_i32(&line, second_digit_index.unwrap());
        result = result + (first_digit * 10 + second_digit);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc() {
        let test_data = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        "#;
        let lines: Vec<String> = test_data.lines()
                                           .map(|line| line.to_string())
                                           .collect();
        let result = aoc_1_1(lines);
        assert_eq!(result, 142, "aoc-1-1 example did not return expected result");
    }
}
