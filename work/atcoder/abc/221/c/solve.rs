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
        mut n: Chars,
    }

    n.sort();
    let mut n = n.iter().map(|&x| (x as u8 - b'0') as usize).collect_vec();

    let mut ans = 0;

    loop {
        // 桁数の差は 1 までなので、各並べ替えにおいて
        // 中間地点で分割して積を求める
        let half = n.len() / 2;
        let a = &n[..half];
        let b = &n[half..];

        // 分割した結果が 0 や、0 から始まるケースは
        // max の比較で捨てられるので特にケアしなくてよい

        let a = parse(a);
        let b = parse(b);

        ans = ans.max(a * b);

        if !n.next_permutation() {
            break;
        }
    }

    println!("{}", ans);
}

fn parse(digits: &[usize]) -> usize {
    digits.iter().fold(0, |acc, &d| acc * 10 + d)
}
