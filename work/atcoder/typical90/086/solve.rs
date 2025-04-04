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
        n: usize, q: usize,
        xyzw: [(Usize1, Usize1, Usize1, usize); q],
    }

    use ac_library::ModInt1000000007 as Mint;

    let mut ans = Mint::new(1);

    for k in 0..60 {
        let mut subans = 0;
        for a in 0..1 << n {
            if xyzw
                .iter()
                .all(|&(x, y, z, w)| (a >> x & 1) | (a >> y & 1) | (a >> z & 1) == w >> k & 1)
            {
                subans += 1;
            }
        }
        ans *= subans;
    }

    println!("{}", ans);
}
