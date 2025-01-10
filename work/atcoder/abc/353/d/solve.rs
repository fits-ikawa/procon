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
        n: usize,
        a: [usize; n],
    }

    use ac_library::ModInt998244353 as Mint;

    let acc = std::iter::once(0)
        .chain(a.iter().copied())
        .cumsum::<usize>()
        .collect_vec();

    // nd[i][j]
    // a の i 番目までの j 桁の数の個数
    let mut nd = vec![vec![0; 11]; n + 1];

    for i in 1..=n {
        for j in 1..=10 {
            nd[i][j] = nd[i - 1][j];
        }

        for j in (1..=10).rev() {
            if a[i - 1] / 10_usize.pow(j - 1) > 0 {
                nd[i][j as usize] += 1;
                break;
            }
        }
    }

    let mut ans = Mint::new(0);

    for i in 0..n - 1 {
        // Ai
        for j in 1..=10 {
            ans += Mint::new(a[i] * 10_usize.pow(j))
                * Mint::new(nd[n][j as usize] - nd[i + 1][j as usize]);
        }

        // Aj
        ans += Mint::new(acc[n] - acc[i + 1]);
    }

    println!("{}", ans);
}
