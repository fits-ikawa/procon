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
        n: usize, k: usize,
    }

    const N: usize = 100000;
    let logk = k.next_power_of_two().ilog2() as usize;

    // ダブリング
    let mut dp = vec![vec![0; N]; logk + 1];

    for j in 0..N {
        dp[0][j] = f(j);
    }

    for i in 1..=logk {
        for j in 0..N {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut cur = n;

    for i in 0..=logk {
        if k >> i & 1 > 0 {
            cur = dp[i][cur];
        }
    }

    println!("{}", cur);
}

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, k: usize,
    }

    let mut set = btreeset! {};
    let mut adj = vec![None; 100000];
    let mut x = n;

    set.insert(n);

    loop {
        let x2 = f(x);
        adj[x] = Some(x2);
        x = x2;

        if set.len() == k {
            println!("{}", x);
            return;
        }

        if set.contains(&x) {
            break;
        }

        set.insert(x);
    }

    let mut cnt = 1;
    let mut cur = x;

    while adj[cur] != Some(x) {
        cnt += 1;
        cur = adj[cur].unwrap();
    }

    for _ in 0..(k - set.len()) % cnt {
        x = f(x);
    }

    println!("{}", x);
}

fn f(x: usize) -> usize {
    let mut z = x;
    let mut y = 0;
    while z > 0 {
        y += z % 10;
        z /= 10;
    }
    (x + y) % 100000
}
