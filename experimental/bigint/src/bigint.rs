extern crate rand;
use rand::Rng;

pub struct BigInt {
    limbs: Vec<u64>,
}

impl BigInt {
    pub fn random(bits: usize) -> Self {
        let num_limbs = (bits + 63) / 64;
        let mut limbs = Vec::with_capacity(num_limbs);
        for _ in 0..bits / 64 {
            limbs.push(rand::thread_rng().gen());
        }
        if bits % 64 != 0 {
            limbs.push(rand::thread_rng().gen::<u64>() & ((1 << (bits % 64)) - 1));
        }
        Self { limbs }
    }

    pub fn binary(&self) -> String {
        self.limbs
            .iter()
            .rev()
            .map(|l| format!("{:064b} ", l))
            .collect::<Vec<String>>()
            .join("")
    }
}
