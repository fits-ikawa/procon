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
        n: usize, m: usize,
        xyz: [(Usize1, Usize1, usize); m],
    }

    let mut adj = vec![vec![]; n];

    for (x, y, z) in xyz {
        adj[x].push((y, z));
        adj[y].push((x, z));
    }

    let mut a = vec![0_usize; n];

    let mut seen = vec![false; n];
    let mut bit = vec![vec![0; 30]; n];

    for i in 0..n {
        if adj[i].is_empty() || seen[i] {
            continue;
        }

        let mut todo = VecDeque::new();
        todo.push_back(i);
        seen[i] = true;

        let mut conn = vec![i];

        while let Some(from) = todo.pop_front() {
            for &(to, z) in &adj[from] {
                for j in 0..30 {
                    if z >> j & 1 == 0 {
                        if !seen[to] {
                            bit[to][j] = bit[from][j];
                        } else if bit[to][j] != bit[from][j] {
                            println!("-1");
                            return;
                        }
                    } else {
                        if !seen[to] {
                            bit[to][j] = (bit[from][j] + 1) % 2;
                        } else if bit[to][j] == bit[from][j] {
                            println!("-1");
                            return;
                        }
                    }
                }
                if !seen[to] {
                    seen[to] = true;
                    todo.push_back(to);
                    conn.push(to);
                }
            }
        }

        for j in 0..30 {
            let s = conn.iter().map(|&i| bit[i][j]).sum::<usize>();
            let t = conn.iter().map(|&i| (bit[i][j] + 1) % 2).sum::<usize>();

            for &i in &conn {
                if s < t {
                    a[i] += bit[i][j] << j;
                } else {
                    a[i] += ((bit[i][j] + 1) % 2) << j;
                }
            }
        }
    }

    println!("{}", a.iter().join(" "));
}
