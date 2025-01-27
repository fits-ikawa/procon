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
    }

    let mut adj = vec![hashset! {}; n];
    let mut ans = n;

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    u: Usize1, v: Usize1,
                }

                adj[u].insert(v);
                adj[v].insert(u);

                if adj[u].len() == 1 {
                    ans -= 1;
                }

                if adj[v].len() == 1 {
                    ans -= 1;
                }
            }
            2 => {
                input! {
                    v: Usize1,
                }

                if !adj[v].is_empty() {
                    ans += 1;

                    let ws = adj[v].iter().copied().collect_vec();

                    for w in ws {
                        adj[w].remove(&v);
                        if adj[w].is_empty() {
                            ans += 1;
                        }
                    }

                    adj[v].clear();
                }
            }
            _ => unreachable!(),
        }
        println!("{}", ans);
    }
}
