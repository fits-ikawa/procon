use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};

#[allow(unused_imports)]
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize, w: usize,
        items: [(usize, usize); n],
    }

    let mut dp = vec![vec![usize::MIN; w + 1]; n + 1];

    for i in 1..=n {
        let (iw, iv) = items[i - 1];
        for j in 0..iw {
            dp[i][j] = dp[i - 1][j];
        }
        for j in iw..=w {
            dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - iw] + iv);
        }
    }

    println!("{}", dp[n][w]);
}
