mod aoc;
use aoc::aoc_3_2;
#[path="../../common/input.rs"]
mod input;
use input::read_stdin_until_eof;

fn main() {
    let lines = read_stdin_until_eof();
    match lines {
        Ok(lines) => {
            println!("{}", aoc_3_2(lines));
        },
        Err(error) => {
            eprintln!("error: {}", error);
            std::process::exit(1);
        }
    }
}
