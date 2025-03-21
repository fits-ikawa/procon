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
        n: usize,
        a: [usize; n],
    }

    let mut dp0 = vec![vec![0; 2]; n + 1];
    let mut dp1 = vec![vec![0; 2]; n + 1];

    dp0[1][1] = usize::MAX;

    dp1[1][0] = usize::MAX;
    dp1[1][1] = a[0];

    for i in 2..=n {
        let ai = a[i - 1];

        dp0[i][0] = dp0[i - 1][1];
        dp0[i][1] = dp0[i - 1][0].min(dp0[i - 1][1]).saturating_add(ai);

        dp1[i][0] = dp1[i - 1][1];
        dp1[i][1] = dp1[i - 1][0].min(dp1[i - 1][1]).saturating_add(ai);
    }

    println!("{}", dp0[n][1].min(dp1[n][0]).min(dp1[n][1]));
}
