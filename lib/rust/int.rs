fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn combination(n: usize, r: usize) -> usize {
    assert!(n >= r);
    let r = r.min(n - r);
    let mut result = 1;
    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn repeated_combination(n: usize, r: usize) -> usize {
    combination(n + r - 1, r)
}

fn permutation(n: usize, r: usize) -> usize {
    assert!(n >= r);
    (n - r + 1..=n).product()
}

fn repeated_permutation(n: usize, r: usize) -> usize {
    n.pow(r as u32)
}

fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    let (mut base, mut exp) = (base, exp);
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp /= 2;
    }
    result
}

fn modulo(a: i64, m: i64) -> i64 {
    let result = a % m;
    if result < 0 {
        result + m
    } else {
        result
    }
}

fn div_ceil(a: i64, b: i64) -> i64 {
    if (a < 0) == (b < 0) {
        (a + b - 1) / b
    } else {
        a / b
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    for i in (5..=(n as f64).sqrt().floor() as u64).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }

    true
}

fn primes_table(n: u64) -> Vec<bool> {
    let n = n as usize;

    if n < 2 {
        return vec![false; n + 1];
    }

    let mut table = vec![true; n + 1];
    table[0] = false;
    table[1] = false;

    for i in 2..=(n as f64).sqrt().floor() as usize {
        if table[i] {
            for j in ((i * i)..=n).step_by(i) {
                table[j] = false;
            }
        }
    }

    table
}

fn primes(n: u64) -> Vec<u64> {
    let table = primes_table(n);
    (2..=n).filter(|&i| table[i as usize]).collect()
}

fn prime_factors(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    }

    let mut factors = vec![];
    let mut n = n;

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    while n % 3 == 0 {
        factors.push(3);
        n /= 3;
    }

    for i in (5..=(n as f64).sqrt().floor() as u64).step_by(6) {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }

        while n % (i + 2) == 0 {
            factors.push(i + 2);
            n /= i + 2;
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

fn count_prime_factors(n: u64) -> Vec<(u64, usize)> {
    prime_factors(n)
        .into_iter()
        .counts()
        .into_iter()
        .sorted()
        .collect::<Vec<_>>()
}

fn divisors(n: u64) -> Vec<u64> {
    let mut asc = vec![];
    let mut desc = vec![];

    for i in 1..=(n as f64).sqrt().floor() as u64 {
        if n % i == 0 {
            asc.push(i);
            if i != n / i {
                desc.push(n / i);
            }
        }
    }

    asc.extend(desc.into_iter().rev());
    asc
}
