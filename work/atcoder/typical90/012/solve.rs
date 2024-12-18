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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        q: usize,
    }

    let mut color = vec![vec![false; w]; h];
    let mut uf = ac_library::Dsu::new(h * w);

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    r: Usize1, c: Usize1,
                }

                color[r][c] = true;

                for (nr, nc) in udir4(r, c) {
                    if nr < h && nc < w && color[nr][nc] {
                        uf.merge(r * w + c, nr * w + nc);
                    }
                }
            }
            2 => {
                input! {
                    ra: Usize1, ca: Usize1, rb: Usize1, cb: Usize1,
                }

                let ans = color[ra][ca] && color[rb][cb] && uf.same(ra * w + ca, rb * w + cb);

                println!("{}", if ans { "Yes" } else { "No" })
            }
            _ => unreachable!(),
        }
    }
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dx, dy)| {
        let new_x = x.wrapping_add(dx);
        let new_y = y.wrapping_add(dy);
        (new_x, new_y)
    })
}
