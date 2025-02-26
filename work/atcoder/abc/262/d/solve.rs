#![allow(clippy::comparison_chain)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    use ac_library::ModInt998244353 as Mint;

    let mut ans = Mint::new(0);

    for m in 1..=n {
        // dp[i][j][k]
        // i 番目までの数字から j 個選んだ和の mod m が k となる通り数
        let mut dp = vec![vec![vec![Mint::new(0); m]; m + 1]; n + 1];
        dp[0][0][0] = Mint::new(1);

        for i in 1..=n {
            let ai = a[i - 1];
            for j in 0..=m {
                for k in 0..m {
                    dp[i][j][k] = dp[i][j][k] + dp[i - 1][j][k];
                    if j < m {
                        let new_k = (k + ai) % m;
                        dp[i][j + 1][new_k] = dp[i][j + 1][new_k] + dp[i - 1][j][k];
                    }
                }
            }
        }

        // 全部の数字から m 個選んで m の倍数になる通り数を計上していく
        ans += dp[n][m][0];
    }

    println!("{}", ans);
}
