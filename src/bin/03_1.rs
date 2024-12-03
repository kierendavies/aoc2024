use std::{
    error::Error,
    io::{self, stdin},
};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let program = io::read_to_string(stdin())?;

    let re = Regex::new(r"mul\((\d{0,3}),(\d{0,3})\)")?;

    let mut sum = 0;

    for capture in re.captures_iter(&program) {
        let (_, [x, y]) = capture.extract();
        let x = x.parse::<i32>()?;
        let y = y.parse::<i32>()?;

        sum += x * y;
    }

    println!("{sum}");

    Ok(())
}
