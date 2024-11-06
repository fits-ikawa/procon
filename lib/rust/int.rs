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

    for i in (5..(n as f64).powf(0.5).floor() as u64).step_by(6) {
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
