mod bigint;
use bigint::BigInt;

fn main() {
    for bits in 61..68 {
        println!("\n{} bits:", bits);
        let mut n: BigInt;
        for _ in 0..4 {
            n = BigInt::random(bits);
            println!("{}", n.binary());
        }
    }
}
