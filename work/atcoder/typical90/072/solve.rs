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
        c: [Chars; h],
    }

    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '.' {
                let mut seen = hashmap! {};
                seen.insert((i, j), 1);
                dfs((i, j), (i, j), &mut seen, &mut ans, h, w, &c);
            }
        }
    }

    if ans > 3 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn dfs(
    cur: (usize, usize),
    goal: (usize, usize),
    seen: &mut HashMap<(usize, usize), usize>,
    ans: &mut usize,
    h: usize,
    w: usize,
    c: &[Vec<char>],
) {
    let (x, y) = cur;
    let cost = seen.get(&cur).copied().unwrap();

    for (nx, ny) in udir4(x, y) {
        if nx < h && ny < w && c[nx][ny] == '.' {
            if (nx, ny) == goal {
                // スタート地点に戻ってきた
                *ans = (*ans).max(cost);
            } else if !seen.contains_key(&(nx, ny)) {
                seen.insert((nx, ny), cost + 1);
                dfs((nx, ny), goal, seen, ans, h, w, c);
                seen.remove(&(nx, ny));
            }
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
