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
        n: usize,
        a: [usize; n],
    }

    let mut seqs = pair_seq(&a); // 0 番目から 2 個ずつペアの連続列を探す
    seqs.extend(pair_seq(&a[1..])); // 1 番目から 2 個ずつペアの連続列を探す

    let mut ans = 0;

    for seq in seqs {
        // 見つけた連続列ごとに尺取り法で最長の 1122 数列を探す
        let mut right = 0;
        let mut nums = hashset! {seq[0]};
        for left in 0..seq.len() {
            while right + 1 < seq.len() && !nums.contains(&seq[right + 1]) {
                right += 1;
                nums.insert(seq[right]);
            }

            ans = ans.max((right - left + 1) * 2);
            nums.remove(&seq[left]);

            if left == right && right + 1 < seq.len() {
                right += 1;
                nums.insert(seq[right]);
            }
        }
    }

    println!("{}", ans);
}

fn pair_seq(a: &[usize]) -> Vec<Vec<usize>> {
    // 頭から 2 個ずつ見ていってペアが連続している部分列を列挙する
    // [2, 2, 3, 3, 4, 5, 5, 6, 7, 7] -> [[2, 3], [7]]
    // 5, 5 の部分は 2 個ずつ見ていくと切れるので列挙されないことに注意
    let mut ret = vec![];
    let mut subret = vec![];

    for c in a.chunks(2) {
        if c.len() == 1 || c[0] != c[1] {
            if !subret.is_empty() {
                ret.push(subret);
                subret = vec![];
            }
        } else {
            subret.push(c[0]);
        }
    }

    if !subret.is_empty() {
        ret.push(subret);
    }

    ret
}
