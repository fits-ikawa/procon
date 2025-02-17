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
    let conn = vec![RefCell::new(btreeset! {}); n];

    for i in 0..n {
        conn[i].borrow_mut().insert(i);
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
                    let mut a = conn[uf.leader(u)].take();
                    let mut b = conn[uf.leader(v)].take();

                    let lnew = uf.merge(u, v);

                    if a.len() >= b.len() {
                        a.extend(b);
                        conn[lnew].replace(a);
                    } else {
                        b.extend(a);
                        conn[lnew].replace(b);
                    };
                }
            }
            2 => {
                input! {
                    v: Usize1, k: Usize1,
                }

                if let Some(w) = conn[uf.leader(v)].borrow().iter().rev().nth(k) {
                    println!("{}", w + 1);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
