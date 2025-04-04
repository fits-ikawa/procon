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
        n: usize, r: usize,
    }

    use ac_library::ModInt1000000007 as Mint;

    let mut a = Mint::new(1);
    let mut b = Mint::new(1);
    let mut c = Mint::new(1);

    for i in 1..=n {
        a *= i;
    }

    for i in 1..=r {
        b *= i;
    }

    for i in 1..=n - r {
        c *= i;
    }

    println!("{}", a / (b * c));
}
