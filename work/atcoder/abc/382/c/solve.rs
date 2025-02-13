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
    // セグ木（累積 min）で解く
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    use ac_library::{Min, Segtree};

    let mut seg = Segtree::<Min<usize>>::new(n);

    for i in 0..n {
        seg.set(i, a[i]);
    }

    for bi in b {
        let pos = seg.max_right(0, |&x| bi < x);

        if pos == n {
            println!("-1");
        } else {
            println!("{}", pos + 1);
        }
    }
}

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
fn solve() {
    // コンテスト中 AC
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut b = b
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .sorted()
        .collect_vec();

    // eaten[i]
    // 料理 i を食べた客の番号
    let mut eaten = vec![-1; m];

    for i in 0..n {
        // 先頭の客から食べられるだけ食べる
        while let Some(&(bj, j)) = b.last() {
            if bj >= a[i] {
                eaten[j] = (i + 1) as isize;
                b.pop();
            } else {
                break;
            }
        }
    }

    println!("{}", eaten.iter().join("\n"));
}
