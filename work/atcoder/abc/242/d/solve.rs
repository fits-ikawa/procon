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
        s: Chars,
        q: usize,
        tk: [(usize, Usize1); q],
    }

    for (t, k) in tk {
        println!("{}", solve(t, k, &s));
    }
}

fn solve(t: usize, k: usize, s: &[char]) -> char {
    if t == 0 {
        return s[k];
    }

    let p = (k + 1).next_power_of_two().ilog2() as usize;
    if t > p {
        // 最初の文字を展開した中に k 番目がある。
        // t が大きいとその展開が長すぎることになるので
        // 2^t が k を下回るように t を減らす
        let nx =
            [['B', 'C'], ['C', 'A'], ['A', 'B']][(s[0] as usize - 'A' as usize + t - p - 1) % 3];
        solve(p, k, &nx)
    } else {
        // 二文字目以降を展開した中に k 番目がある。
        // t は 60 未満なので愚直に展開していく
        let q = 2_usize.pow(t as u32);
        let nth = k / q;
        solve(t - 1, k % q, &next(s[nth]))
    }
}

fn next(c: char) -> [char; 2] {
    match c {
        'A' => ['B', 'C'],
        'B' => ['C', 'A'],
        'C' => ['A', 'B'],
        _ => unreachable!(),
    }
}
