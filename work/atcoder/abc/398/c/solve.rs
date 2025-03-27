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
    }

    let mut n2i = hashmap! {};

    for i in 0..n {
        n2i.insert(a[i], i);
    }

    let cnt = a.iter().counts();

    let mut mx = None;

    for (&k, v) in cnt {
        if v == 1 {
            mx = mx.max(Some(k));
        }
    }

    if let Some(m) = mx {
        println!("{}", n2i[&m] + 1);
    } else {
        println!("-1");
    }
}
