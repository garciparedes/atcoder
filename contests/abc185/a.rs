use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split_whitespace()
        .map(|a_i| a_i.parse::<i32>().unwrap())
        .min()
        .unwrap();

    println!("{:?}", ans);

    return Ok(());
}
