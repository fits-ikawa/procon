#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering::*, Reverse};
use std::{collections::*, hash};
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
    // クエリ 1 を 1 回でも実行する前と後で場合分け
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    }

    let mut all = None;
    let mut add = hashmap! {};

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: usize,
                }

                all = Some(x);
                add.clear();
            }
            2 => {
                input! {
                    i: Usize1, x: usize,
                }

                if all.is_none() {
                    a[i] += x;
                } else {
                    *add.entry(i).or_insert(0) += x;
                }
            }
            3 => {
                input! {
                    i: Usize1,
                }

                if all.is_none() {
                    println!("{}", a[i]);
                } else {
                    println!("{}", all.unwrap() + add.get(&i).unwrap_or(&0));
                }
            }
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
fn solve() {
    // クエリ 1 の実行にかかわらず処理を一般化。でも時間がかかる。
    // n が大きいときの HashMap がやはり遅い？
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut base = 0;
    let mut diff = a.into_iter().enumerate().collect::<HashMap<_, _>>();

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: usize,
                }

                base = x;
                diff.clear();
            }
            2 => {
                input! {
                    i: Usize1, x: usize,
                }

                *diff.entry(i).or_insert(0) += x;
            }
            3 => {
                input! {
                    i: Usize1,
                }

                println!("{}", base + diff.get(&i).unwrap_or(&0));
            }
            _ => unreachable!(),
        }
    }
}
