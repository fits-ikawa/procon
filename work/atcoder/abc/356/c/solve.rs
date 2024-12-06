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
    // ビット演算版
    input! {
        n: usize, m: usize, k:u32,
    }

    let mut c = vec![];
    let mut a = vec![];
    let mut r = vec![];

    for _ in 0..m {
        input! {
            ci: usize,
            ai: [Usize1; ci],
            ri: char,
        }
        c.push(ci);

        let mut bits = 0;
        for j in 0..ci {
            bits += 1_usize << ai[j];
        }

        a.push(bits);
        r.push(ri == 'o');
    }

    let mut ans = 0;

    for comb in 0..1 << n {
        if (0..m).all(|i| ((comb & a[i]).count_ones() >= k) == r[i]) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // HashSet 版
    input! {
        n: usize, m: usize, k:usize,
    }

    let mut c = vec![];
    let mut a = vec![];
    let mut r = vec![];

    for _ in 0..m {
        input! {
            ci: usize,
            ai: [Usize1; ci],
            ri: char,
        }
        c.push(ci);
        a.push(ai.into_iter().collect::<HashSet<_>>());
        r.push(ri == 'o');
    }

    let mut ans = 0;

    for i in 0..=n {
        for comb in (0..n).combinations(i) {
            let comb = comb.into_iter().collect::<HashSet<_>>();
            let mut consistent = true;

            for j in 0..m {
                let expected = comb.intersection(&a[j]).count() >= k;
                consistent &= expected == r[j];
            }

            if consistent {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
