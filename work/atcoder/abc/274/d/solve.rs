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
        n: usize, x: isize, y: isize,
        a: [usize; n],
    }

    let a_even = a.iter().step_by(2).copied().collect_vec();
    let a_odd = a[1..].iter().step_by(2).copied().collect_vec();

    let ans =
        calc(&a_even, (x + 10000) as usize, true) && calc(&a_odd, (y + 10000) as usize, false);

    println!("{}", if ans { "Yes" } else { "No" });
}

fn calc(a: &[usize], x: usize, is_row: bool) -> bool {
    let n = a.len();

    // dp[i][j]
    // i 番目までの数の +/- の組み合わせで総和を j にできるかどうか
    let mut dp = vec![vec![false; 20001]; n + 1];
    dp[0][10000] = true;
    if is_row {
        // 横方向は一手目が指定されている
        dp[1][10000 + a[0]] = true;
    }

    for i in (if is_row { 2 } else { 1 })..=n {
        let ai = a[i - 1];
        for j in 0..=20000 {
            if dp[i - 1][j] {
                dp[i][j + ai] = true;
                dp[i][j - ai] = true;
            }
        }
    }

    dp[n][x]
}
