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
        n: usize, m: usize, p: usize, q: usize, r: usize,
        xyz: [(Usize1, Usize1, usize); r],
    }

    let mut x2yz = vec![vec![]; n];

    for (x, y, z) in xyz {
        x2yz[x].push((y, z));
    }

    let mut ans = 0;

    for i in 0_usize..1 << n {
        if i.count_ones() as usize == p {
            let mut cs = vec![0; m];
            for j in 0..n {
                if i & 1 << j > 0 {
                    for &(y, z) in &x2yz[j] {
                        cs[y] += z;
                    }
                }
            }
            cs.sort();
            cs.reverse();
            ans = ans.max(cs.iter().take(q).sum::<usize>());
        }
    }

    println!("{}", ans);
}
