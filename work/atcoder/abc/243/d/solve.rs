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
    // 事前に移動を最適化してからシミュレート（解説 AC）
    input! {
        _n: usize, x: usize,
        s: Chars,
    }

    let mut reduced = vec![];

    for si in s {
        if si == 'U' {
            if reduced.is_empty() || reduced.last().copied().unwrap() == 'U' {
                reduced.push('U');
            } else {
                reduced.pop();
            }
        } else {
            reduced.push(si);
        }
    }

    let mut cur = x;

    for ri in reduced {
        cur = match ri {
            'U' => cur / 2,
            'L' => cur * 2,
            'R' => cur * 2 + 1,
            _ => unreachable!(),
        };
    }

    println!("{}", cur);
}

#[allow(dead_code)]
fn solve() {
    // 愚直シミュレート
    // 深いところでは階層の上下のみ記録
    input! {
        _n: u128, x: u128,
        s: Chars,
    }

    let border = 1152921504606846976_u128; // 2^60
    let mut cur = x;
    let mut deep = false;
    let mut tier = 0;

    for si in s {
        if !deep {
            cur = match si {
                'U' => cur / 2,
                'L' => cur * 2,
                'R' => cur * 2 + 1,
                _ => unreachable!(),
            };
            deep = cur >= border;
            tier = 1;
        } else {
            tier = match si {
                'U' => tier - 1,
                'L' => tier + 1,
                'R' => tier + 1,
                _ => unreachable!(),
            };
            if tier == 0 {
                cur /= 2;
                deep = false;
            }
        }
    }

    println!("{}", cur);
}
