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
        n: usize, s: f64, t: f64,
        abcd: [(f64, f64, f64, f64); n],
    }

    let mut seen = vec![false; n];

    let ans = dfs(0.0, 0.0, n, s, t, &abcd, &mut seen);

    println!("{}", ans);
}

fn dfs(
    x: f64,
    y: f64,
    n: usize,
    s: f64,
    t: f64,
    abcd: &[(f64, f64, f64, f64)],
    seen: &mut [bool],
) -> f64 {
    if n == 0 {
        return 0.0;
    }

    let mut ans = f64::MAX;

    for (i, &(a, b, c, d)) in abcd.iter().enumerate() {
        if seen[i] {
            continue;
        }
        seen[i] = true;

        // x, y から a, b まで移動
        let mut ab = distance(x, y, a, b) / s;
        // a, b から c, d まで印字
        ab += distance(a, b, c, d) / t;
        // 残った線分を印字
        ab += dfs(c, d, n - 1, s, t, abcd, seen);

        // x, y から c, d まで移動（逆順の印字を試す）
        let mut cd = distance(x, y, c, d) / s;
        // c, d から a, b まで印字
        cd += distance(c, d, a, b) / t;
        // 残った線分を印字
        cd += dfs(a, b, n - 1, s, t, abcd, seen);

        // 最も速いケースを採用
        ans = ans.min(ab.min(cd));

        seen[i] = false;
    }

    ans
}

fn distance(a: f64, b: f64, c: f64, d: f64) -> f64 {
    ((c - a).powi(2) + (d - b).powi(2)).sqrt()
}
