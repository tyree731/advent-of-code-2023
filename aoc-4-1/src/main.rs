mod aoc;
use aoc::aoc_4_1;
#[path="../../common/input.rs"]
mod input;
use input::read_stdin_until_eof;

fn main() {
    let lines = read_stdin_until_eof();
    match lines {
        Ok(lines) => {
            println!("{}", aoc_4_1(lines));
        },
        Err(error) => {
            eprintln!("error: {}", error);
            std::process::exit(1);
        }
    }
}
