// use crate::bigint::BigInt;
mod bigint;
use bigint::BigInt;

fn generate_prime(n_bits: usize) -> BigInt {
    let mut n = BigInt::from_u64(0);
    while true {
        n = BigInt::random(n_bits);
        n |= 1;
        println!("{}", n.binary());
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
    let mut n1 = n; n1.clone().minus_one();
    let mut d = n1.clone();
    let s = d.trailing_zeros();
    d >>= s;

    for i in 0..k {
        println!("round {}", i);
        let a = BigInt::random(n.bit_length()-1);
        let mut x = a.modpow(&d, n);
        if x == 1 || x == *n1 {
            continue;
        }

        if s > 0 {
            for _ in 0..(s - 1) {
                x = x.modpow_u32(2, n);
                if x == *n1 {
                    break;
                }
            }
        }

        if x != *n1 {
            println!("{}\n{}", x.binary(), n1.binary());
            println!("exit {}", x.binary());
            return false;
        }
    }

    return true;
}

fn main() {
    let n = generate_prime(64);
    println!("Generated prime: {}", n.binary());
}
