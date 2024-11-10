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
        n: usize,
        a: [Chars; n],
    }

    let mut buf = vec![vec!['?'; n]; n];

    for i in 1..=n / 2 {
        let j = i % 4;
        for (x, y) in
            iproduct!(i..=n + 1 - i, [i, n + 1 - i]).chain(iproduct!([i, n + 1 - i], i..=n + 1 - i))
        {
            let (to_x, to_y) = if j == 1 {
                // 90度回転
                (y, n + 1 - x)
            } else if j == 2 {
                // 180度回転
                (n + 1 - x, n + 1 - y)
            } else if j == 3 {
                // 270度回転 (-90度回転)
                (n + 1 - y, x)
            } else {
                // 無回転
                (x, y)
            };
            buf[to_x - 1][to_y - 1] = a[x - 1][y - 1];
        }
    }

    println!(
        "{}",
        buf.iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n")
    );
}
