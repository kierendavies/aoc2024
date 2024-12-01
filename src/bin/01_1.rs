use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line_res in stdin().lines() {
        let line = line_res?;

        let mut tokens = line.split_whitespace();
        left.push(
            tokens
                .next()
                .ok_or("unexpected end of line")?
                .parse::<i32>()?,
        );
        right.push(
            tokens
                .next()
                .ok_or("unexpected end of line")?
                .parse::<i32>()?,
        );
    }

    left.sort_unstable();
    right.sort_unstable();

    let total_distance = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    println!("{total_distance}");

    Ok(())
}
