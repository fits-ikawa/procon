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
    // あまり速度変わらず
    input! {
        n: usize, k: usize,
        s: Chars,
    }

    let mut next = vec![vec![n; 26]; n];

    for i in (0..n).rev() {
        if i < n - 1 {
            for j in 0..26 {
                next[i][j] = next[i + 1][j];
            }
        }
        next[i][(s[i] as u8 - b'a') as usize] = i;
    }

    let mut ans = vec![];
    let mut cur = 0;

    while ans.len() < k {
        for i in 0..26 {
            let pos = next[cur][i];
            if pos < n && pos <= n - k + ans.len() {
                ans.push((b'a' + i as u8) as char);
                cur = pos + 1;
                break;
            }
        }
    }

    println!("{}", ans.iter().collect::<String>());
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, k: usize,
        s: Chars,
    }

    let mut idx = vec![vec![]; 26];

    for (i, si) in s.into_iter().enumerate() {
        idx[(si as u8 - b'a') as usize].push(i);
    }

    let mut ans = vec![];
    let mut next = 0;

    while ans.len() < k {
        for i in 0..26 {
            let pos = idx[i].lower_bound(&next);
            if pos < idx[i].len() && idx[i][pos] <= n - k + ans.len() {
                ans.push((b'a' + i as u8) as char);
                next = idx[i][pos] + 1;
                break;
            }
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
