use std::io::prelude::*;
use std::io;
use std::cmp;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer
        .trim()
        .split('\n');

    let head: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let n = head[0];
    let t = head[2];


    let visits = lines
        .map(|line| {
            let mut line = line.trim().split_whitespace().map(|x| x.parse::<u64>().unwrap());
            let a = line.next().unwrap();
            let b = line.next().unwrap();
            return (a, b);
        });

    let mut ans = true;
    let mut last = 0;
    let mut current = n;
    for (a, b) in visits {
        if a - last >= current {
            ans = false;
            break;
        }
        current = cmp::min(n, current - (a - last) + b - a); 
        last = b;
    }

    if t - last >= current {
        ans = false;
    }

    println!("{}", if ans { "Yes" } else { "No" });

    return Ok(());
}
