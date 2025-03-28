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
        n: usize, m: usize,
        mut a: [isize; n],
    }

    let mut vals = vec![vec![]; m + 1];

    for i in 0..n {
        if a[i] > n as isize {
            continue;
        }

        let l = if a[i] >= 0 {
            1
        } else {
            (a[i].unsigned_abs() + i) / (i + 1)
        };

        for j in l..=m {
            let v = (a[i] + (i as isize + 1) * (j as isize)) as usize;
            if v > n {
                break;
            }
            vals[j].push(v);
        }
    }

    for i in 1..=m {
        let k = vals[i].len();
        let mut used = vec![false; k + 1];

        for &v in &vals[i] {
            if v <= k {
                used[v] = true;
            }
        }

        let mex = (0..).find(|&j| !used[j]).unwrap();

        println!("{}", mex);
    }
}
