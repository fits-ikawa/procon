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
        b: [usize; n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    }

    let mut m = 0;
    let mut map = hashmap! {};

    for i in 0..n {
        if !map.contains_key(&a[i]) {
            map.insert(a[i], m);
            m += 1;
        }
    }

    for i in 0..n {
        if !map.contains_key(&b[i]) {
            map.insert(b[i], m);
            m += 1;
        }
    }

    let aa = a.iter().map(|&x| map[&x]).collect_vec();
    let bb = b.iter().map(|&x| map[&x]).collect_vec();

    let mut a_set = hashset! {};
    let mut b_set = hashset! {};

    let mut a_kind = vec![0; n];
    let mut b_kind = vec![0; n];
    let mut b_max = vec![0; n];
    let mut mx = 0;

    for i in 0..n {
        a_set.insert(&aa[i]);
        b_set.insert(&bb[i]);

        a_kind[i] = a_set.len();
        b_kind[i] = b_set.len();
        mx = mx.max(bb[i]);
        b_max[i] = mx;
    }

    for (x, y) in xy {
        let ans = a_kind[x] == b_kind[y] && a_kind[x] - 1 == b_max[y];
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
