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
        n: usize, k: usize,
        hs: [usize; n],
    }

    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;

    for i in 0..n {
        for j in i + 1..=min(i + k, n - 1) {
            dp[j] = min(dp[j], dp[i] + hs[i].abs_diff(hs[j]));
        }
    }

    println!("{}", dp.last().unwrap());
}
