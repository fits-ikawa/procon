#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    // 解説 AC
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }

    let m = a.iter().max().copied().unwrap();
    let mut s = vec![0; m + 1];

    for &ai in &a {
        s[ai] += 1;
    }

    let mut t = vec![0; m + 1];

    for i in 1..=m {
        for j in (i..=m).step_by(i) {
            t[i] += s[j];
        }
    }

    let mut u = vec![0; m + 1];

    // i は gcd を走査
    for i in 1..=m {
        if t[i] < k {
            // i の倍数が k 個未満なので
            // i を gcd とする k 個以上の要素は取れない
            continue;
        }

        // 個数の条件はクリアしたので i の倍数に対して i を答えとして記録。
        // i を昇順に走査しているので最大値で上書きされていく
        for j in (i..=m).step_by(i) {
            u[j] = i;
        }
    }

    for ai in a {
        println!("{}", u[ai]);
    }
}

#[allow(dead_code)]
fn solve() {
    // 嘘解法っぽい
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }

    // 約数の列挙
    let mut div = vec![vec![]; 1000001];

    for i in 1..=1000000 {
        let mut j = 1;
        while i * j <= 1000000 {
            div[i * j].push(i);
            j += 1;
        }
    }

    let mut cnt = vec![0; 1000001];

    for &ai in &a {
        for &m in &div[ai] {
            cnt[m] += 1;
        }
    }

    for ai in a {
        for &m in div[ai].iter().rev() {
            if cnt[m] >= k {
                println!("{}", m);
                break;
            }
        }
    }
}
