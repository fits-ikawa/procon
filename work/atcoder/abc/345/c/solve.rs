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
        s: Chars,
    }

    // max: 重複を気にしない場合の交換の組み合わせ数
    let n = s.len();
    let max = n * (n - 1) / 2;

    // 各文字の出現回数
    let count = s.iter().sorted().copied().dedup_with_count().collect_vec();

    // s[i] == s[j] なら交換しても意味がない。
    // 2 回以上出現する文字 c の出現回数が k 回なら
    // s[i] == s[j] == c となるような i, j の組み合わせ数は
    // k * (k - 1) / 2 なので、この値を max から引く
    let mut ans = max
        - count
            .iter()
            .map(|&(v, _)| if v >= 2 { v * (v - 1) / 2 } else { 0 })
            .sum::<usize>();

    // 上で s[i] == s[j] の交換をすべて除外したが、どの文字でもいいので 1 回だけは
    // 元と同じ文字列を生成する交換としてカウントする必要がある
    if count.iter().map(|&(v, _)| v).max().unwrap() >= 2 {
        ans += 1;
    }

    println!("{}", ans);
}
