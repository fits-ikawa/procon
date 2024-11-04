use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};

#[allow(unused_imports)]
use std::cmp::{max, min};

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        mat: [[usize; 3]; n],
    }

    let mut dp = vec![vec![usize::MIN; 3]; n];
    dp[0][0] = mat[0][0];
    dp[0][1] = mat[0][1];
    dp[0][2] = mat[0][2];

    for i in 1..n {
        dp[i][0] = max(dp[i - 1][1] + mat[i][0], dp[i - 1][2] + mat[i][0]);
        dp[i][1] = max(dp[i - 1][0] + mat[i][1], dp[i - 1][2] + mat[i][1]);
        dp[i][2] = max(dp[i - 1][0] + mat[i][2], dp[i - 1][1] + mat[i][2]);
    }

    println!("{}", dp.last().unwrap().iter().max().unwrap());
}
