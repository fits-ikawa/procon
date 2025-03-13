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
        n: u64, d: u64,
    }

    use ac_library::ModInt998244353 as Mint;

    let mint2 = Mint::new(2);
    let mut ans = Mint::new(0);

    for i in 0..n - 1 {
        if (n - 1 - i) * 2 < d {
            break;
        }

        if n - 1 - i >= d {
            ans += mint2.pow(d) * mint2.pow(i) * 2;
        }

        if d > 1 {
            let left = (n - 1 - i).min(d - 1);
            let right = d - left;
            ans +=
                mint2.pow(left - 1) * mint2.pow(right - 1) * (left - right + 1) * mint2.pow(i) * 2;
        }
    }

    println!("{}", ans);
}
