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
        n: usize, m: usize,
        x: [Usize1; m],
    }

    let mut imos = vec![0_isize; n + 1];

    for (&u, &v) in x.iter().tuple_windows() {
        let (mut u, mut v) = (u, v);
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }

        let d1 = (v - u) as isize;
        let d2 = n as isize - d1;

        imos[u] += d2;
        imos[v] -= d2;

        imos[0] += d1;
        imos[u] -= d1;
        imos[v] += d1;
        imos[n] -= d1;
    }

    let acc = imos.iter().cumsum::<isize>().collect_vec();
    let ans = acc[..n].iter().min().copied().unwrap();

    println!("{}", ans);
}
