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
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n*2],
        }

        let mut next = hashset! {};

        for i in 0..n * 2 - 1 {
            if a[i] == a[i + 1] {
                next.insert(a[i]);
            }
        }

        let mut cand = hashmap! {};
        let mut ans = 0;

        for i in 0..n * 2 - 1 {
            if next.contains(&a[i]) || next.contains(&a[i + 1]) {
                continue;
            }

            if let Some(&j) = cand.get(&(a[i], a[i + 1])) {
                if i != j {
                    ans += 1;
                }
            } else {
                cand.insert((a[i], a[i + 1]), i + 1);
                cand.insert((a[i + 1], a[i]), i + 1);
            }
        }

        println!("{}", ans);
    }
}
