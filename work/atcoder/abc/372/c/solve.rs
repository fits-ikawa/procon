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
        _n: usize, q: usize,
        mut s: Chars,
    }

    // 番兵
    s.insert(0, 'X');
    s.insert(0, 'X');
    s.push('X');
    s.push('X');

    let mut abc = hashset![];

    // "ABC" の数を 'A' の位置で管理
    for i in 0..s.len() - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            abc.insert(i);
        }
    }

    for _ in 0..q {
        input! {
            x: Usize1, c: char,
        }

        let x = x + 2; // 番兵のぶん、ずらす
        s[x] = c;

        // 変更位置を含む長さ 3 の部分文字列が "ABC" になるかチェック
        for i in x - 2..=x {
            if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                abc.insert(i);
            } else {
                abc.remove(&i);
            }
        }
        println!("{}", abc.len());
    }
}
