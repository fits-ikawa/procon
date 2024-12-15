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
        n: usize, s: usize,
        a: [usize; n],
    }

    let sum_a = a.iter().sum::<usize>();
    let s = s % sum_a;

    if s == 0 {
        println!("Yes");
        return;
    }

    let a2 = a.iter().chain(a.iter()).copied().collect_vec();
    let m = a2.len();

    let mut right = 0;
    let mut sum = a2[0];

    for left in 0..m {
        while right + 1 < m && sum + a2[right + 1] <= s {
            right += 1;
            sum += a2[right];
        }

        if sum == s {
            println!("Yes");
            return;
        }
        sum -= a2[left];

        if left == right && right + 1 < m {
            right += 1;
            sum = a2[right];
        }
    }

    println!("No");
}
