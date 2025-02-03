mod bigint;
use bigint::BigInt;

fn generate_prime(n_bits: usize) -> BigInt {
    let mut n;
    // Use an infinite loop to generate candidates.
    loop {
        n = BigInt::random(n_bits);
        n |= 1; // Ensure the number is odd.
        if miller_rabin(&n, 40) {
            break;
        }
    }
    n
}

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

    for i in 0..k {
        let a = BigInt::random(n.bit_length() - 1);
        n.assert_valid();
        let mut x = a.modpow(&d, n);

        if x == 1 || x == n1 {
            continue;
        }

        for _ in 0..(s - 1) {
            x = x.modpow_u32(2, n);
            if x == n1 {
                break;
            }
        }

        if x != n1 {
            return false;
        }
    }

    true
}

fn main() {
    let n = generate_prime(128);
    println!("0x{}", n.hex());
}
