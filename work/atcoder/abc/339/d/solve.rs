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
        n: usize,
        mut s: [Chars; n],
    }

    let mut ps = vec![];

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                ps.push((i, j));
                s[i][j] = '.';
            }
        }
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![vec![vec![vec![None; n]; n]; n]; n];

    todo.push_back((ps[0], ps[1]));
    seen[ps[0].0][ps[0].1][ps[1].0][ps[1].1] = Some(0);

    while let Some((a, b)) = todo.pop_front() {
        for (dx, dy) in [(!0, 0), (1, 0), (0, !0), (0, 1)] {
            let moved = [a, b]
                .into_iter()
                .map(|(x, y)| {
                    let nx = x.wrapping_add(dx);
                    let ny = y.wrapping_add(dy);
                    if nx < n && ny < n && s[nx][ny] != '#' {
                        (nx, ny)
                    } else {
                        (x, y)
                    }
                })
                .collect_vec();

            if moved[0] == moved[1] {
                println!("{}", seen[a.0][a.1][b.0][b.1].unwrap() + 1);
                return;
            }

            if seen[moved[0].0][moved[0].1][moved[1].0][moved[1].1].is_none() {
                seen[moved[0].0][moved[0].1][moved[1].0][moved[1].1] =
                    seen[a.0][a.1][b.0][b.1].map(|e| e + 1);
                todo.push_back((moved[0], moved[1]));
            }
        }
    }

    println!("-1");
}
