#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cell::RefCell;
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
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut conn = vec![btreeset! {}; n];

    for i in 0..n {
        conn[i].insert(i);
    }

    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    u: Usize1, v: Usize1,
                }

                if !uf.same(u, v) {
                    let mut a = std::mem::take(&mut conn[uf.leader(u)]);
                    let mut b = std::mem::take(&mut conn[uf.leader(v)]);

                    let lnew = uf.merge(u, v);

                    if a.len() >= b.len() {
                        a.extend(b);
                        conn[lnew] = a;
                    } else {
                        b.extend(a);
                        conn[lnew] = b;
                    };
                }
            }
            2 => {
                input! {
                    v: Usize1, k: Usize1,
                }

                if let Some(w) = conn[uf.leader(v)].iter().rev().nth(k) {
                    println!("{}", w + 1);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
