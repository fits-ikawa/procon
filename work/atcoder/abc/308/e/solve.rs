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
        s: Chars,
    }

    // left_m[j][ai]
    // 0 <= i <= j となる i において s[i] == "M", a[i] == ai になるものの個数
    let mut left_m = vec![vec![0; 3]; n];

    // right_x[j][ak]
    // j <= k < n となる k において s[k] == "X", a[k] == ak になるものの個数
    let mut right_x = vec![vec![0; 3]; n];

    for i in 0..n {
        let rev_i = n - i - 1;

        if i > 0 {
            for j in 0..3 {
                left_m[i][j] = left_m[i - 1][j];
                right_x[rev_i][j] = right_x[rev_i + 1][j];
            }
        }

        if s[i] == 'M' {
            left_m[i][a[i]] += 1;
        }

        if s[rev_i] == 'X' {
            right_x[rev_i][a[rev_i]] += 1;
        }
    }

    let mex = |ai, aj, ak| {
        for m in 0..=3 {
            if m != ai && m != aj && m != ak {
                return m;
            }
        }
        unreachable!();
    };

    let mut ans = 0;

    for j in 0..n {
        if s[j] == 'E' {
            for ai in 0..3 {
                for ak in 0..3 {
                    ans += mex(ai, a[j], ak) * left_m[j][ai] * right_x[j][ak];
                }
            }
        }
    }

    println!("{}", ans);
}
