mod aoc;
use aoc::aoc_2_2;
use std::io::{stdin, BufRead};

fn read_stdin_until_eof() -> Result<Vec<String>, String> {
    let mut lines: Vec<String> = Vec::new();
    let stdin = stdin().lock();
    for line in stdin.lines() {
        match line {
            Ok(line) => {
                lines.push(line);
            },
            Err(_) => {
                if lines.is_empty() {
                    return Err(String::from("stdin was empty"));
                }
                break;
            }
        }
    }
    return Ok(lines);
}

fn main() {
    let lines = read_stdin_until_eof();
    match lines {
        Ok(lines) => {
            println!("{}", aoc_2_2(lines));
        },
        Err(error) => {
            eprintln!("error: {}", error);
            std::process::exit(1);
        }
    }
}
