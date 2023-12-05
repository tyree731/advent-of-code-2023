fn parse_lines_to_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    return lines.iter().filter(|line| {
        return !line.is_empty();
    }).map(|line| {
        return line.chars().collect();
    }).collect()
}

fn extract_number(grid: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> u32 {
    let mut left_bound = col_index;
    let mut right_bound = col_index + 1;
    while left_bound > 0 {
        if char::is_ascii_digit(&grid[row_index][left_bound - 1]) {
            left_bound -= 1;
        } else {
            break;
        }
    }
    let grid_width = grid[0].len();
    while right_bound < grid_width {
        if char::is_ascii_digit(&grid[row_index][right_bound]) {
            right_bound += 1;
        } else {
            break;
        }
    }
    let number_string: String = grid[row_index][left_bound..right_bound].iter().collect();
    return number_string.parse().unwrap();
}

fn extract_adjacent_numbers(grid: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();

    // check left
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    if col_index > 0 && char::is_ascii_digit(&grid[row_index][col_index - 1]) {
        results.push(extract_number(grid, row_index, col_index - 1));
    }

    // check right
    if col_index < grid_width - 1 && char::is_ascii_digit(&grid[row_index][col_index + 1]) {
        results.push(extract_number(grid, row_index, col_index + 1));
    }

    // handle above
    if row_index > 0 {
        let considered_row = row_index - 1;
        if char::is_ascii_digit(&grid[considered_row][col_index]) {
            results.push(extract_number(grid, considered_row, col_index));
        } else {
            if col_index > 0 && char::is_ascii_digit(&grid[considered_row][col_index - 1]) {
                results.push(extract_number(grid, considered_row, col_index - 1));
            }
            if col_index < grid_width - 1 && char::is_ascii_digit(&grid[considered_row][col_index+1]) {
                results.push(extract_number(grid, considered_row, col_index + 1));
            }
        }
    }

    // handle below
    if row_index < grid_height - 1 {
        let considered_row = row_index + 1;
        if char::is_ascii_digit(&grid[considered_row][col_index]) {
            results.push(extract_number(grid, considered_row, col_index));
        } else {
            if col_index > 0 && char::is_ascii_digit(&grid[considered_row][col_index - 1]) {
                results.push(extract_number(grid, considered_row, col_index - 1));
            }
            if col_index < grid_width - 1 && char::is_ascii_digit(&grid[considered_row][col_index + 1]) {
                results.push(extract_number(grid, considered_row, col_index + 1));
            }
        }
    }

    return results;
}

fn extract_gear_ratios(grid: Vec<Vec<char>>) -> Vec<u32> {
    if grid.is_empty() {
        return Vec::new();
    }

    // first, lets find all gears
    let mut gears: Vec<(usize, usize)> = Vec::new();
    for (row_index, line) in grid.iter().enumerate() {
        for (col_index, c) in line.iter().enumerate() {
            if *c == '*' {
                gears.push((row_index, col_index));
            }
        }
    }

    // second, filter gears by those with two numbers
    // adjacent to them, and multiply and add those numbers
    // if there are just two
    let mut results: Vec<u32> = Vec::new();
    for gear in gears {
        let adjacent_numbers = extract_adjacent_numbers(&grid, gear.0, gear.1);
        if adjacent_numbers.len() == 2 {
            results.push(adjacent_numbers[0] * adjacent_numbers[1]);
        }
    }

    return results;
}

pub fn aoc_3_2(lines: Vec<String>) -> u32 {
    let grid = parse_lines_to_grid(lines);
    let numbers = extract_gear_ratios(grid);
    return numbers.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_3_2() {
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
        let result = aoc_3_2(lines);
        assert_eq!(result, 467835, "aoc-3-2 example did not return expected result");
    }
}
