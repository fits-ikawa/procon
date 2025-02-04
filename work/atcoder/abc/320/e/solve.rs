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
        tws: [(usize, usize, usize); m],
    }

    let mut people = BTreeSet::from_iter(0..n);
    let mut event = btreeset! {};

    for (t, w, s) in tws {
        event.insert((t, true, w, s));
    }

    let mut ans = vec![0; n];

    while let Some(e) = event.pop_first() {
        match e.1 {
            true => {
                // そうめんを得る
                let (t, _, w, s) = e;

                if let Some(p) = people.pop_first() {
                    ans[p] += w;
                    event.insert((t + s, false, p, 0));
                }
            }
            false => {
                // 人が列に戻る
                let (_, _, p, _) = e;
                people.insert(p);
            }
        }
    }

    println!("{}", ans.iter().join("\n"));
}
