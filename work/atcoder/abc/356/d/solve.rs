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
    }

    use ac_library::ModInt998244353 as Mint;

    let mut ans = Mint::new(0);

    for i in 0..60 {
        if m & 1 << i > 0 {
            let base = 2_usize.pow(i + 1);
            ans += (n / base) * (base / 2);
            ans += (n % base).saturating_sub(base / 2 - 1);
        }
    }

    println!("{}", ans);
}
