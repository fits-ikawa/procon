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
        r: Chars,
        c: Chars,
    }

    let mut available = vec![(0..n).collect::<HashSet<_>>(); 3];

    if !dfs(0, &mut available, &mut vec![], n, &r, &c) {
        println!("No");
    }
}

fn dfs(
    i: usize,
    a: &mut [HashSet<usize>],
    b: &mut Vec<Vec<char>>,
    n: usize,
    r: &[char],
    c: &[char],
) -> bool {
    if i == n {
        // ここに到達したら b は正しい書き込み方になっている
        println!("Yes");
        println!("{}", b.iter().map(|line| line.iter().join("")).join("\n"));

        return true;
    }

    for perm in (0..n).permutations(3) {
        if (0..3).all(|j| a[j].contains(&perm[j])) {
            for j in 0..3 {
                a[j].remove(&perm[j]);
            }

            let mut row = vec!['.'; n];
            row[perm[0]] = 'A';
            row[perm[1]] = 'B';
            row[perm[2]] = 'C';

            b.push(row);

            let check = || {
                // 行の条件を確認
                for y in 0..n {
                    if b[i][y] != '.' {
                        if b[i][y] != r[i] {
                            return false;
                        } else {
                            break;
                        }
                    }
                }

                // 列の条件を確認
                for y in 0..n {
                    for x in 0..i + 1 {
                        if b[x][y] != '.' {
                            if b[x][y] != c[y] {
                                return false;
                            } else {
                                break;
                            }
                        }
                    }
                }

                true
            };

            if check() && dfs(i + 1, a, b, n, r, c) {
                return true;
            }

            b.pop();

            for j in 0..3 {
                a[j].insert(perm[j]);
            }
        }
    }

    false
}
