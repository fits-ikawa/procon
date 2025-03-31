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
        n: usize, l: usize, r: usize,
        x: [usize; n],
    }

    use ac_library::{Min, Segtree};

    let mut seg = Segtree::<Min<usize>>::new(n);
    seg.set(0, 0);

    for i in 1..n {
        let xi = x[i];

        let left = xi.saturating_sub(r);
        let right = xi.saturating_sub(l);

        let pos_l = x.lower_bound(&left);
        let pos_r = x.upper_bound(&right);

        let c = seg.prod(pos_l..pos_r);

        if c < usize::MAX {
            seg.set(i, c + 1);
        }
    }

    println!("{}", seg.get(n - 1));
}
