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
        n: usize, l: isize, r: isize,
        a: [isize; n],
    }

    let asum = a.iter().sum::<isize>();

    // lacc[i]/racc[i]
    // 左/右から i 番目までを l/r で置き換えたときの a の総和の変化
    let mut lacc = vec![0; n + 1];
    let mut racc = vec![0; n + 1];

    for i in 0..n {
        lacc[i + 1] = lacc[i] + l - a[i];
        racc[i + 1] = racc[i] + r - a[n - 1 - i];
    }

    // rmin[i]
    // 右から i 番目以下までを r で置き換えたときの a の総和の変化の最小の値
    // （言い換え）右から i 番目以下までを r で置き換えらえるとき、
    //            a の総和をもっとも減らせるのは rmin[i] 変化させたとき
    let mut rmin = vec![0; n + 1];
    let mut min = 0;

    for i in 1..=n {
        min = min.min(racc[i]);
        rmin[i] = min;
    }

    let mut ans = isize::MAX;

    for i in 0..=n {
        ans = ans.min(asum + lacc[i] + rmin[n - i]);
    }

    println!("{}", ans);
}
