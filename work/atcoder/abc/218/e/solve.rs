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
        mut abc: [(Usize1, Usize1, isize); m],
    }

    abc.sort_by_key(|&(_, _, c)| c);

    let mut uf = ac_library::Dsu::new(n);
    let mut ans = 0;

    for (a, b, c) in abc {
        if uf.same(a, b) {
            if c >= 0 {
                ans += c;
            }
        } else {
            uf.merge(a, b);
        }
    }

    println!("{}", ans);
}
