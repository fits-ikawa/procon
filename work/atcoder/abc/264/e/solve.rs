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
        n: usize, m: usize, e: usize,
        uv: [(Usize1, Usize1); e],
        q: usize,
        x: [Usize1; q],
    }

    let x_set = x.iter().copied().collect::<HashSet<_>>();
    let mut uf = ac_library::Dsu::new(n + m);

    for (u, v) in (0..e).filter_map(|i| {
        if x_set.contains(&i) {
            None
        } else {
            Some(uv[i])
        }
    }) {
        uf.merge(u, v);
    }

    let mut e = vec![(0, false); n + m];
    let mut n_elec_city = 0;

    for mut g in uf.groups() {
        let leader = uf.leader(g[0]);
        g.sort();
        let n_city = g.lower_bound(&n);
        let elec = g.len() > n_city;

        e[leader] = (n_city, elec);

        if elec {
            n_elec_city += n_city;
        }
    }

    let mut ans = Vec::with_capacity(q);

    for &xi in x.iter().rev() {
        ans.push(n_elec_city);

        let (u, v) = uv[xi];

        if !uf.same(u, v) {
            let lu = uf.leader(u);
            let lv = uf.leader(v);
            let (nu, eu) = e[lu];
            let (nv, ev) = e[lv];

            if eu && !ev {
                n_elec_city += nv;
            } else if !eu && ev {
                n_elec_city += nu;
            }

            let lnew = uf.merge(u, v);

            e[lnew] = (nu + nv, eu | ev);
        }
    }

    println!("{}", ans.iter().rev().join("\n"));
}
