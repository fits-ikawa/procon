#![allow(clippy::comparison_chain)]
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
    input! {
        n: usize,
        a: [usize; n],
    }

    // 解き直し

    const LOGA: usize = 27;

    let mut ans = 0;

    for k in 0..LOGA {
        let b = a.iter().map(|&ai| ai >> k & 1).collect_vec();

        let mut c = vec![0; n - 1];
        c[0] = b[0] ^ b[1];

        for i in 1..n - 1 {
            c[i] = c[i - 1] ^ b[i + 1];
        }

        // d[0][i] が 2^k なら d[1][i] は 0（またはその逆）
        // …のように値がフリップしている
        let mut d = vec![vec![0; n]; 2];

        for i in 1..n {
            d[0][i] = d[0][i - 1] + (c[i - 1] << k);
            d[1][i] = d[1][i - 1] + ((c[i - 1] ^ 1) << k);
        }

        let mut j = 0;

        for i in 0..n - 1 {
            ans += d[j][n - 1] - d[j][i];
            // 次のループでは b[i] が使われなくなるが、
            // この値が 1 なら（xor の結果が反転するので）使う値をフリップさせる
            j ^= b[i];
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let loga = a.iter().max().unwrap().next_power_of_two().ilog2();
    let mut ans = 0;

    for k in 0..=loga {
        let b = a.iter().map(|&ai| ai >> k & 1).collect_vec();
        let mut c = vec![0; b.len() + 1];

        for i in 0..b.len() {
            c[i + 1] = c[i] ^ b[i];
        }

        let mut d = vec![vec![0; c.len() + 1]; 2];

        for i in 0..c.len() {
            d[0][i + 1] = d[0][i] + (c[i] + 1) % 2;
            d[1][i + 1] = d[1][i] + c[i];
        }

        let mut subans = 0;

        for i in 0..n - 1 {
            let j = (c[i] + 1) % 2;
            subans += d[j][n + 1] - d[j][i + 2];
        }

        ans += subans * 2_usize.pow(k);
    }

    println!("{}", ans);
}
