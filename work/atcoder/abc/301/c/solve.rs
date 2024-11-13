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
        s: String,
        t: String,
    }

    let mut s_count = s.chars().counts();
    let mut t_count = t.chars().counts();

    s_count.entry('@').or_insert(0);
    t_count.entry('@').or_insert(0);

    for c in "atcoder".chars() {
        // 変換なしで a, t, c, o, d, e, r の個数を相殺
        s_count.entry(c).or_insert(0);
        t_count.entry(c).or_insert(0);
        let n = s_count[&c].min(t_count[&c]);
        *s_count.get_mut(&c).unwrap() -= n;
        *t_count.get_mut(&c).unwrap() -= n;

        if s_count[&c] > 0 {
            // S の a, t, c, o, d, e, r を T の @ と相殺
            let m = s_count[&c].min(t_count[&'@']);
            *s_count.get_mut(&c).unwrap() -= m;
            *t_count.get_mut(&'@').unwrap() -= m;
        }
        if t_count[&c] > 0 {
            // T の a, t, c, o, d, e, r を S の @ と相殺
            let m = t_count[&c].min(s_count[&'@']);
            *t_count.get_mut(&c).unwrap() -= m;
            *s_count.get_mut(&'@').unwrap() -= m;
        }
    }

    // 文字列を復元
    let s2 = s_count
        .into_iter()
        .flat_map(|(k, v)| vec![k; v])
        .sorted()
        .collect::<String>();
    let t2 = t_count
        .into_iter()
        .flat_map(|(k, v)| vec![k; v])
        .sorted()
        .collect::<String>();

    println!("{}", if s2 == t2 { "Yes" } else { "No" });
}
