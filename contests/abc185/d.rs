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
    let m = head[1];
    if m == 0 {
        println!("1");
        return Ok(());
    }
    let mut blue: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    blue.push(0);
    blue.push(n + 1);

    blue.sort_unstable();
   
    
    let mut k = u64::max_value();
    for i in 1..blue.len(){
        let d = blue[i] - (blue[i - 1] + 1);
        if d < 1 {
            continue;
        }
        k = cmp::min(k, d);
    }

    let mut ans = 0; 
    for i in 1..blue.len() {
        let d = blue[i] - (blue[i - 1] + 1);
        if d < 1 {
            continue;
        }
            ans += (d as f64 / k as f64).ceil() as u64;
    }

    println!("{:?}", ans);

    return Ok(());
}
