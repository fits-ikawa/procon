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
        mut s: [Chars; h],
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                for (x, y) in udir4(i, j) {
                    if x < h && y < w && s[x][y] == '#' {
                        s[i][j] = '*';
                    }
                }
            }
        }
    }

    let mut seen = vec![vec![false; w]; h];
    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '*' {
                ans = ans.max(1);
            } else if s[i][j] == '.' && !seen[i][j] {
                let mut todo = VecDeque::new();
                let mut cnt = 0;
                let mut stars = hashset! {};

                todo.push_back((i, j));
                seen[i][j] = true;

                while let Some((x, y)) = todo.pop_front() {
                    cnt += 1;
                    for (nx, ny) in udir4(x, y) {
                        if nx < h && ny < w {
                            if s[nx][ny] == '.' && !seen[nx][ny] {
                                seen[nx][ny] = true;
                                todo.push_back((nx, ny));
                            } else if s[nx][ny] == '*' {
                                stars.insert((nx, ny));
                            }
                        }
                    }
                }

                ans = ans.max(cnt + stars.len());
            }
        }
    }

    println!("{}", ans);
}

const UDIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn udir4(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    UDIR4.iter().map(move |&(dx, dy)| {
        let new_x = x.wrapping_add(dx);
        let new_y = y.wrapping_add(dy);
        (new_x, new_y)
    })
}
