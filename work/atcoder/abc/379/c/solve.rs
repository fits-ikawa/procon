#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering, Reverse};
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
fn main() {
    input! {
        n: usize, m: usize,
        x: [i64; m],
        a: [i64; m],
    }

    if a.iter().sum::<i64>() != n as i64 {
        println!("-1");
        return;
    }

    let stones = izip!(x, a)
        .sorted_unstable_by_key(|&(a, _)| Reverse(a))
        .collect::<Vec<_>>();

    let mut last_one = n as i64 + 1;
    let mut ans = 0;

    for (i, stone) in stones {
        let zeros = last_one - i - 1;
        let diff = stone - zeros;

        if diff < 0 {
            // 右側を埋めるのに石が足りない
            // 右側に 0 のマスが 5 個で石が 3 個あるなら
            // 5 + 4 + 3 回の移動が必要
            ans += (zeros + zeros - stone + 1) * stone / 2;
            last_one = i + diff.abs() + 1;
        } else if diff > 1 {
            // 右側を埋めても石が余る
            println!("-1");
            return;
        } else {
            // 右側を 1 個ずつで埋められる
            // 右側に 0 のマスが 5 個なら
            // 5 + 4 + 3 + 2 + 1 回の移動が必要
            ans += (zeros + 1) * zeros / 2;
            last_one = i + 1 - diff; // i に石が 1 個残るかどうか
        }
    }

    println!("{}", ans);
}
