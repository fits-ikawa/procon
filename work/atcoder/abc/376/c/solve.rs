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
        mut a: [usize; n],
        mut b: [usize; n-1],
    }

    a.sort_by_key(|&x| Reverse(x));
    b.sort_by_key(|&x| Reverse(x));
    b.push(0); // 番兵

    let mut j = 0;
    let mut pick = 0;

    for i in 0..n {
        // おもちゃと箱を大きい順に入るかどうかマッチング
        if a[i] > b[i - j] {
            // 入らない場合、1 個だけは保留できる
            j += 1;
            pick = i;
            if j > 1 {
                println!("-1");
                return;
            }
        }
    }

    println!("{}", a[pick]);
}
