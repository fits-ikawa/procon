#![allow(clippy::comparison_chain)]
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
    }

    let mut a = vec![];
    let mut xy = vec![];

    for _ in 0..n {
        input! {
            ai: usize,
            xyi: [(Usize1, usize); ai],
        }

        a.push(ai);
        xy.push(xyi);
    }

    let mut ans = 0;

    for set in (0..n).powerset() {
        let set = set.into_iter().collect::<HashSet<_>>();

        if set.iter().all(|&i| {
            xy[i].iter().all(|&(x, y)| {
                if y == 1 {
                    set.contains(&x)
                } else {
                    !set.contains(&x)
                }
            })
        }) {
            ans = ans.max(set.len());
        }
    }
    println!("{}", ans);
}
