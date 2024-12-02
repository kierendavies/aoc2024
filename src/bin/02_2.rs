#![feature(array_windows)]

use std::{cmp::Reverse, error::Error, io::stdin, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let mut reports = Vec::new();

    for line_res in stdin().lines() {
        let line = line_res?;

        let report = line
            .split_whitespace()
            .map(i32::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        reports.push(report);
    }

    let safe_count = reports
        .iter()
        .filter(|report| report_is_safe(report))
        .count();

    println!("{safe_count}");

    Ok(())
}

fn report_is_safe(report: &[i32]) -> bool {
    if report_is_safe_strict(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report = report.to_vec();
        report.remove(i);

        if report_is_safe_strict(&report) {
            return true;
        }
    }

    false
}

fn report_is_safe_strict(report: &[i32]) -> bool {
    let monotonic = report.is_sorted() || report.is_sorted_by_key(Reverse);

    let gradual = report
        .array_windows::<2>()
        .map(|[a, b]| (b - a).abs())
        .all(|diff| (1..=3).contains(&diff));

    monotonic && gradual
}
