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
        n: usize, m: usize,
        a: [Usize1; m],
    }

    let mut b2i = (0..n).collect_vec();
    let mut i2b = (0..n).collect_vec();
    let mut swap = vec![];

    for i in 0..m {
        let x = i2b[a[i]];
        let y = i2b[a[i] + 1];

        swap.push((x, y));
        b2i.swap(x, y);
        i2b.swap(a[i], a[i] + 1);
    }

    let mut ans = vec![];

    for (x, y) in swap {
        b2i.swap(x, y);
        ans.push(b2i[0] + 1);
        b2i.swap(x, y);
    }

    println!("{}", ans.iter().join("\n"));
}
