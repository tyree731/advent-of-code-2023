use regex::Regex;

struct Handful {
    reds: u32,
    greens: u32,
    blues: u32,
}

struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

fn extract_if_matched(cube_str: &str, regex: &Regex) -> u32 {
    match (*regex).captures(cube_str) {
        Some(captures) => {
            if let Some(matched) = captures.get(1) {
                return matched.as_str().parse().unwrap();
            } else {
                return 0;
            }
        },
        None => {
            return 0;
        }
    }
}

fn extract_game(line: String) -> Option<Game> {
    if line.is_empty() {
        return None;
    }

    // grab the game id, and separate out the handfuls
    let game_id: u32;
    let handfuls_str: &str;
    let initial_regex = Regex::new("^Game ([[:digit:]]+): (.*)$").unwrap();
    match initial_regex.captures(&line) {
        Some(captures) => {
            game_id = captures.get(1).unwrap().as_str().parse().unwrap();
            handfuls_str = captures.get(2).unwrap().as_str();
        },
        None => {
            return None;
        }
    }

    // split the handfuls on ;, extract reds, blues and greens
    let reds_regex = Regex::new("^[[:space:]]*([[:digit:]]+) red[[:space:]]*$").unwrap();
    let greens_regex = Regex::new("^[[:space:]]*([[:digit:]]+) green[[:space:]]*$").unwrap();
    let blues_regex = Regex::new("^[[:space:]]*([[:digit:]]+) blue[[:space:]]*$").unwrap();

    let mut handfuls: Vec<Handful> = Vec::new();
    let handful_strs: Vec<&str> = handfuls_str.split(";").collect();
    for handful_str in handful_strs {
        let cube_strs: Vec<&str> = handful_str.split(",").collect();
        let mut red_count: u32 = 0;
        let mut blue_count: u32 = 0;
        let mut green_count: u32 = 0;
        for cube_str in cube_strs {
            let possible_red_count: u32 = extract_if_matched(cube_str, &reds_regex);
            if possible_red_count > 0 {
                red_count = possible_red_count;
            }
            let possible_blue_count: u32 = extract_if_matched(cube_str, &blues_regex);
            if possible_blue_count > 0 {
                blue_count = possible_blue_count;
            }
            let possible_green_count: u32 = extract_if_matched(cube_str, &greens_regex);
            if possible_green_count > 0 {
                green_count = possible_green_count;
            }
        }
        handfuls.push(Handful { reds: red_count, greens: green_count, blues: blue_count })
    }
    return Some(Game { id: game_id, handfuls: handfuls });
}

pub fn aoc_2_1(lines: Vec<String>, reds: u32, greens: u32, blues: u32) -> u32 {
    let mut id_sum: u32 = 0;
    for line in lines {
        match extract_game(line) {
            Some(game) => {
                let mut found: bool = true;
                for handful in &game.handfuls {
                    if handful.reds > reds || handful.greens > greens || handful.blues > blues {
                        found = false;
                        break;
                    }
                }
                if found {
                    id_sum += game.id;
                }
            },
            None => {
                continue;
            }
        }
    }
    return id_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_2_1() {
        let test_data = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        let lines: Vec<String> = test_data.lines()
                                           .map(|line| line.to_string())
                                           .collect();
        let result = aoc_2_1(lines, 12, 13, 14);
        assert_eq!(result, 8, "aoc-2-1 example did not return expected result");
    }
}
