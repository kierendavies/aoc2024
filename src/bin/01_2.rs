use std::{collections::HashMap, error::Error, io::stdin};

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

    let mut right_count = HashMap::new();
    for r in right {
        right_count.entry(r).and_modify(|c| *c += 1).or_insert(1);
    }

    let similarity_score = left
        .into_iter()
        .map(|l| {
            let c = right_count.get(&l).copied().unwrap_or(0);
            l * c
        })
        .sum::<i32>();

    println!("{similarity_score}");

    Ok(())
}
