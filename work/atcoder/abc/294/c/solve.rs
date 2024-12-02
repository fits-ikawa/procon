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
    // 解説 AC
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let c = a.iter().chain(b.iter()).sorted().copied().collect_vec();

    // 二分探索 1 回の計算量が O(logN) なので結局 O(NlogN) となり同じ
    let a_pos = a.iter().map(|x| c.lower_bound(x) + 1).collect_vec();
    let b_pos = b.iter().map(|x| c.lower_bound(x) + 1).collect_vec();

    println!("{}", a_pos.iter().join(" "));
    println!("{}", b_pos.iter().join(" "));
}

#[allow(dead_code)]
fn solve() {
    // ガチャガチャやってしまった版
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    use AorB::*;

    let a = a.into_iter().map(|x| (x, A)).collect_vec();
    let b = b.into_iter().map(|x| (x, B)).collect_vec();

    let mut c = [a, b].concat();
    c.sort_by_key(|x| x.0);

    let mut resume_a = vec![];
    let mut resume_b = vec![];

    for (i, (x, aorb)) in c.into_iter().enumerate() {
        match aorb {
            A => resume_a.push((x, i + 1)),
            B => resume_b.push((x, i + 1)),
        }
    }

    resume_a.sort();
    resume_b.sort();

    println!("{}", resume_a.iter().map(|&(_, i)| i).join(" "));
    println!("{}", resume_b.iter().map(|&(_, i)| i).join(" "));
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum AorB {
    A,
    B,
}
