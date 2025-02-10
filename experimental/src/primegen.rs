mod bigint;
use bigint::BigInt;

/// Miller-Rabin primality test.
fn miller_rabin(n: &BigInt, k: u64) -> bool {
    n.assert_valid();
    if n < &2u64 {
        return false;
    }
    if n == &2u64 || n == &3u64 {
        return true;
    }
    if n.is_even() {
        return false;
    }

    let mut n1 = n.clone();
    n1.minus_one();
    let mut d = n1.clone();
    let s = d.trailing_zeros();
    d >>= s;

    for _ in 0..k {
        let a = BigInt::random(n.bit_length() - 1);
        n.assert_valid();
        let mut x = a.modpow(&d, n);

        if x == 1 || x == n1 {
            continue;
        }

        let mut passed_round = false;
        for _ in 0..((s as usize) - 1) {
            x = x.modpow_u32(2, n);
            if x == n1 {
                passed_round = true;
                break;
            }
        }

        if !passed_round {
            return false;
        }
    }
    true
}

/// Generates a prime with the given bit length.
fn generate_prime(n_bits: usize, primes: &[u64]) -> BigInt {
    loop {
        let mut candidate = BigInt::random(n_bits);
        candidate |= 1; // Ensure the candidate is odd.
        if primes.iter().any(|&prime| &candidate % prime == 0) {
            continue;
        }
        if miller_rabin(&candidate, 40) {
            return candidate;
        }
    }
}

fn generate_first_1000_smallest_primes() -> Vec<u64> {
    let mut primes = Vec::with_capacity(1000);
    let mut candidate: u64 = 2;
    while primes.len() < 1000000 {
        let mut is_prime = true;
        let sqrt_candidate = (candidate as f64).sqrt() as u64;
        for &p in &primes {
            if p > sqrt_candidate {
                break;
            }
            if candidate % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(candidate);
        }
        candidate += 1;
    }
    primes
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_bits: usize = args
        .iter()
        .find(|arg| arg.starts_with("--n_bits="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(128);
    let repeat: usize = args
        .iter()
        .find(|arg| arg.starts_with("--repeat="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    BigInt::seed(42);
    println!("Generating {} primes with {} bits", repeat, n_bits);

    let primes = generate_first_1000_smallest_primes();
    for _ in 0..repeat {
        let prime = generate_prime(n_bits, &primes);
        println!("0x{}", prime.hex());
    }
}
