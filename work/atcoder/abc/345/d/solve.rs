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
        n: usize, h: usize, w: usize,
        ab: [(usize, usize); n],
    }

    for perm in ab.into_iter().permutations(n) {
        let mut free = iproduct!((0..h), (0..w)).collect::<BTreeSet<_>>();

        if dfs(0, &mut free, &perm, &mut vec![], n, h, w) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

#[derive(Debug, Clone, Copy, Hash)]
struct Rect {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

// AABB テスト
fn test(a: &Rect, b: &Rect) -> bool {
    a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top
}

fn dfs(
    i: usize,
    free: &mut BTreeSet<(usize, usize)>,
    tiles: &[(usize, usize)],
    board: &mut Vec<Rect>,
    n: usize,
    h: usize,
    w: usize,
) -> bool {
    if free.is_empty() {
        return true;
    }

    if i == n {
        return false;
    }

    let mut th = tiles[i].0;
    let mut tw = tiles[i].1;

    let (x, y) = free.first().copied().unwrap();

    for _ in 0..2 {
        if x + th <= h && y + tw <= w {
            let rect = Rect {
                left: y,
                top: x,
                right: y + tw,
                bottom: x + th,
            };

            if board.iter().all(|r| !test(r, &rect)) {
                board.push(rect);

                for x in rect.top..rect.bottom {
                    for y in rect.left..rect.right {
                        debug_assert!(free.contains(&(x, y)));
                        free.remove(&(x, y));
                    }
                }

                if dfs(i + 1, free, tiles, board, n, h, w) {
                    return true;
                }

                for x in rect.top..rect.bottom {
                    for y in rect.left..rect.right {
                        free.insert((x, y));
                    }
                }

                board.pop();
            }
        }

        if th == tw {
            break;
        }

        std::mem::swap(&mut th, &mut tw);
    }

    false
}
