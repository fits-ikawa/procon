#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        n: usize, k: usize,
        a: [usize; n],
    }

    if n == 1 {
        println!("{}", if a[0] == k { "Yes" } else { "No" });
        return;
    }

    let b = [a[..n / 2].to_vec(), a[n / 2..].to_vec()];
    let mut c = [Vec::with_capacity(1 << 15), Vec::with_capacity(1 << 15)];

    for i in 0..=1 {
        for choice in 0..1 << b[i].len() {
            let mut sum = 0;
            for j in 0..b[i].len() {
                if choice >> j & 1 > 0 {
                    sum += b[i][j];
                }
            }
            c[i].push(sum);
        }
    }

    c[1].sort();

    for &x in &c[0] {
        if x <= k && c[1].binary_search(&(k - x)).is_ok() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
