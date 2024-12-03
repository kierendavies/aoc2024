use core::str;
use std::{
    io::{self, stdin},
    str::FromStr,
};

use anyhow::{anyhow, Context, Error};
use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Instr {
    Do,
    Dont,
    Mul(i32, i32),
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            b"do()" => Ok(Self::Do),
            b"don't()" => Ok(Self::Dont),
            [b'm', b'u', b'l', b'(', args @ .., b')'] => {
                let mut args_iter = str::from_utf8(args).unwrap().split(',');

                let mut parse_arg = || -> Result<i32, Error> {
                    let arg_str = args_iter.next().context("missing argument")?;

                    let arg = arg_str.parse()?;
                    Ok(arg)
                };

                let x = parse_arg()?;
                let y = parse_arg()?;

                if args_iter.next().is_some() {
                    return Err(anyhow!("too many arguments"));
                }

                Ok(Self::Mul(x, y))
            }
            _ => Err(anyhow!("invalid instruction")),
        }
    }
}

fn main() -> Result<(), Error> {
    let program = io::read_to_string(stdin())?;

    let re = Regex::new(r"mul\(\d{0,3},\d{0,3}\)|do\(\)|don't\(\)")?;

    let mut mul_enabled = true;
    let mut sum = 0;

    for instr_match in re.find_iter(&program) {
        let instr = instr_match.as_str().parse()?;

        match instr {
            Instr::Do => mul_enabled = true,
            Instr::Dont => mul_enabled = false,
            Instr::Mul(x, y) => {
                if mul_enabled {
                    sum += x * y;
                }
            }
        }
    }

    println!("{sum}");

    Ok(())
}
