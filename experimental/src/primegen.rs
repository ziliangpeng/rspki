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
    return true;
}

fn main() {
    let n = generate_prime(64);
    println!("Generated prime: {}", n.binary());
}
