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
        n: usize, m: usize, l: usize,
        a: [usize; n],
        b: [usize; m],
        cd: [(Usize1, Usize1); l],
    }

    let a = a
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, x)| x)
        .rev()
        .collect_vec();

    let b = b
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, x)| x)
        .rev()
        .collect_vec();

    let cd = cd.into_iter().collect::<HashSet<_>>();

    let mut todo = BinaryHeap::new();
    let mut seen = hashset! {};

    todo.push((a[0].1 + b[0].1, 0, 0, a[0].0, b[0].0));
    seen.insert((0, 0));

    while let Some((price, i, j, ai, bi)) = todo.pop() {
        if !cd.contains(&(ai, bi)) {
            println!("{}", price);
            return;
        }

        if i + 1 < n && !seen.contains(&(i + 1, j)) {
            seen.insert((i + 1, j));
            todo.push((a[i + 1].1 + b[j].1, i + 1, j, a[i + 1].0, b[j].0));
        }

        if j + 1 < m && !seen.contains(&(i, j + 1)) {
            seen.insert((i, j + 1));
            todo.push((a[i].1 + b[j + 1].1, i, j + 1, a[i].0, b[j + 1].0));
        }
    }
}
