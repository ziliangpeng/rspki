// use crate::bigint::BigInt;
mod bigint;
use bigint::BigInt;

fn generate_prime(n_bits: usize) -> BigInt {
    let mut n = BigInt::from_u64(0);
    while true {
        n = BigInt::random(n_bits);
        n |= 1;
        if miller_rabin(&n, 40) {
            break;
        }
    }
    n
}

fn miller_rabin(n: &BigInt, k: u64) -> bool {
    if n < &2u64 {
        return false;
    }
    if n == &2u64 || n == &3u64 {
        return true;
    }
    if n.is_even() {
        return false;
    }

    // let mut s = BigInt::from_u64(0);
    let mut n1 = n; n1.minus_one();
    let d = n1;
    let s = d.trailing_zeros();
    d >>= s;

    for _ in 0..k {
        let a = BigInt::random(n_bits);
        let x = a.modpow(d, n);
        if x == 1 || x == n1 {
            continue;
        }

        for _ in 0..(s - 1) {
            x = x.modpow(2, n);
            if x == n1 {
                break;
            }
        }

        if x != n1 {
            return false;
        }
    }

    return true;
}

fn main() {
    let n = generate_prime(64);
    println!("Generated prime: {}", n.binary());
}
