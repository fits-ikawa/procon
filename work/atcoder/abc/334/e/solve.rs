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
        h: usize, w: usize,
        a: [Chars; h],
    }

    let mut uf = ac_library::Dsu::new(h * w);
    let mut g = vec![];

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                g.push(i * w + j);
            }
            for (x, y) in udir4(i, j) {
                if x < h && y < w && a[i][j] == '#' && a[x][y] == '#' {
                    uf.merge(i * w + j, x * w + y);
                }
            }
        }
    }

    let n = g.into_iter().map(|v| uf.leader(v)).unique().count();
    let mut p = 0;
    let mut q = 0;

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '.' {
                let mut next_g = hashset! {};

                for (x, y) in udir4(i, j) {
                    if x < h && y < w && a[x][y] == '#' {
                        next_g.insert(uf.leader(x * w + y));
                    }
                }

                p += n - next_g.len() + 1;
                q += 1;
            }
        }
    }

    use ac_library::ModInt998244353 as Mint;

    println!("{}", Mint::new(p) / q);
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dx, dy)| {
        let new_x = x.wrapping_add(dx);
        let new_y = y.wrapping_add(dy);
        (new_x, new_y)
    })
}
