mod bigint;
use bigint::BigInt;

fn main() {
    for bits in [62, 63, 64, 65, 66, 67] {
        println!("\n{} bits:", bits);
        let mut n = BigInt::random(bits);
        println!("{}", n.binary());
        n = BigInt::random(bits);
        println!("{}", n.binary());
        n = BigInt::random(bits);
        println!("{}", n.binary());
    }
}
