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
        mut a: [usize; n],
        ix: [(Usize1, usize); q],
    }

    let mut set = BTreeSet::from_iter(0..=n);
    let mut cnt = vec![0; n];

    for &ai in &a {
        if ai < n {
            cnt[ai] += 1;
            set.remove(&ai);
        }
    }

    for (i, x) in ix {
        if a[i] < n {
            cnt[a[i]] -= 1;
            if cnt[a[i]] == 0 {
                set.insert(a[i]);
            }
        }

        if x < n {
            cnt[x] += 1;
            set.remove(&x);
        }

        a[i] = x;

        println!("{}", set.first().unwrap());
    }
}
