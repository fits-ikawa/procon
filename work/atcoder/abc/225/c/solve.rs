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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        b: [[Usize1; m]; n],
    }

    let offset = b[0][0] % 7;

    for i in 0..n {
        // 縦方向の整合性確認
        if i > 0 && b[i][0] != b[i - 1][0] + 7 {
            println!("No");
            return;
        }

        // 横方向の整合性確認
        for j in 0..m {
            if j > 0 && b[i][j] != b[i][j - 1] + 1 {
                println!("No");
                return;
            }
            if b[i][j] % 7 != offset + j {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
