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
        q: usize,
        lr: [(Usize1, Usize1); q],
    }

    let acc = std::iter::once(0).chain(a).cumsum::<usize>().collect_vec();

    for (l, r) in lr {
        let win = acc[r + 1] - acc[l];
        let lose = r + 1 - l - win;

        println!(
            "{}",
            match win.cmp(&lose) {
                Greater => "win",
                Less => "lose",
                Equal => "draw",
            }
        );
    }
}
