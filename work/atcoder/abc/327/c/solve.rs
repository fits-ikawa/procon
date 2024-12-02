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
        a: [[usize; 9]; 9],
    }

    // 横に見る
    let mut check = a.clone();

    // 縦に見る
    for i in 0..9 {
        let mut col = vec![];
        for j in 0..9 {
            col.push(a[j][i]);
        }
        check.push(col);
    }

    // 3x3 ブロックで見る
    for i in 0..3 {
        for j in 0..3 {
            let mut block = vec![];
            for k in 0..9 {
                block.push(a[i * 3 + k / 3][j * 3 + k % 3]);
            }
            check.push(block);
        }
    }

    let ans = check
        .iter()
        .all(|c| c.iter().collect::<HashSet<_>>().len() == 9);

    println!("{}", if ans { "Yes" } else { "No" });
}
