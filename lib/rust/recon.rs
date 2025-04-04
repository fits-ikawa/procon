fn recon(n: u32, modulus: i64) -> (i64, i64) {
    let n = n as i64;
    let mut v = (modulus, 0);
    let mut w = (n, 1);

    while w.0 * w.0 * 2 > modulus {
        let q = v.0 / w.0;
        let z = (v.0 - q * w.0, v.1 - q * w.1);
        v = w;
        w = z;
    }

    if w.1 < 0 {
        w = (-w.0, -w.1);
    }

    w
}

fn recon_998244353(n: u32) -> (i64, i64) {
    recon(n, 998244353)
}

fn recon_1000000007(n: u32) -> (i64, i64) {
    recon(n, 1000000007)
}
