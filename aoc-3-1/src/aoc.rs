use colored::Colorize;

fn parse_lines_to_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    return lines.iter().filter(|line| {
        return !line.is_empty();
    }).map(|line| {
        return line.chars().collect();
    }).collect()
}

fn extract_numbers(grid: Vec<Vec<char>>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let grid_length: i32 = grid.len() as i32;
    for (row_index, line) in grid.iter().enumerate() {
        let mut current_index: i32 = 0;
        let line_length = line.len() as i32;
        loop {
            while current_index < line_length {
                if char::is_ascii_digit(&line[current_index as usize]) {
                    break;
                }
                print!("{}", &line[current_index as usize]);
                current_index += 1;
            }
            if current_index >= line_length {
                // no more numbers on this line
                println!("");
                break;
            }

            // number found, extract its positions
            let start_index: i32 = current_index;
            while current_index < line_length && char::is_ascii_digit(&line[current_index as usize]) {
                current_index += 1;
            }

            // number goes from start_index to just before current_index, in row row_index
            // search around there for symbols
            let row_begin: i32 = std::cmp::max(row_index as i32 -1, 0);
            let col_begin: i32 = std::cmp::max(start_index-1, 0);
            let row_end: i32 = std::cmp::min(row_index as i32 + 2, grid_length);
            let col_end: i32 = std::cmp::min(current_index+1, line_length);
            let mut found = false;
            'outer: for row in row_begin..row_end {
                for col in col_begin..col_end {
                    if row == row_index as i32 && col >= start_index && col < current_index {
                        // skip values in our string
                        continue;
                    }
                    let value = grid[row as usize][col as usize];
                    if value != '.' && char::is_ascii_punctuation(&value) {
                        // we found a symbol, add the value to the vec and move on
                        found = true;
                        break 'outer;
                    }
                }
            }
            if found {
                let number_string: String = line[start_index as usize..current_index as usize].iter().collect();
                let number: u32 = number_string.parse().unwrap();
                result.push(number_string.parse().unwrap());
                print!("{}", number);
            } else {
                let number_string: String = line[start_index as usize..current_index as usize].iter().collect();
                print!("{}", number_string.red());
            }
        }
    }
    return result;
}

pub fn aoc_3_1(lines: Vec<String>) -> u32 {
    let grid = parse_lines_to_grid(lines);
    let numbers = extract_numbers(grid);
    return numbers.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_3_1() {
        let test_data = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        "#;
        let lines: Vec<String> = test_data.lines()
                                           .map(|line| line.to_string())
                                           .collect();
        let result = aoc_3_1(lines);
        assert_eq!(result, 4361, "aoc-3-1 example did not return expected result");
    }
}
