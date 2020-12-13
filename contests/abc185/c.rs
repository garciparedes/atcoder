use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let l = buffer
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut memo = HashMap::new();
    let ans = helper(l, 0, 0, &mut memo);

    println!("{:?}", ans);

    return Ok(());
}

fn helper(l: usize, cuts: usize, i: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if cuts == 11 && i < l {
        return 1;
    }
    if let Some(&ans) = memo.get(&(cuts, i)) {
        return ans;
    }    


    let mut ans = 0;
    for j in i..l {
        ans += helper(l, cuts + 1, j + 1, memo);
    }

    memo.insert((cuts, i), ans);
    return ans;
}
