extern crate rand;
use rand::Rng;
use std::ops::Add;
use std::ops::BitAnd;
use std::ops::BitOrAssign;

pub struct BigInt {
    limbs: Vec<u64>, // binary-based limbs. each limb represents a 2^64 block.
}

// Construction and instantiation
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

    pub fn from_binary(binary: &str) -> Self {
        let limbs = binary
            .chars()
            .rev()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| u64::from_str_radix(&chunk.iter().rev().collect::<String>(), 2).unwrap())
            .collect::<Vec<u64>>();
        Self { limbs }
    }

    pub fn from_hex(hex: &str) -> Self {
        let limbs = hex
            .chars()
            .rev()
            .collect::<Vec<char>>()
            .chunks(16)
            .map(|chunk| u64::from_str_radix(&chunk.iter().rev().collect::<String>(), 16).unwrap())
            .collect::<Vec<u64>>();
        Self { limbs }
    }

    pub fn from_u64(n: u64) -> Self {
        Self { limbs: vec![n] }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut limbs = Vec::with_capacity(self.limbs.len() + other.limbs.len() + 1);
        let mut carry = 0;
        for i in 0..self.limbs.len() {
            let (sum, overflow) = self.limbs[i].overflowing_add(other.limbs[i]);
            limbs.push(sum + carry);
            carry = overflow as u64;
        }
        if carry != 0 {
            limbs.push(carry);
        }
        Self { limbs }
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.limbs == other.limbs
    }
}

impl PartialEq<u64> for BigInt {
    fn eq(&self, other: &u64) -> bool {
        if self.limbs.len() > 1 {
            return false;
        }
        self.limbs[0] == *other
    }
}

impl PartialOrd<u64> for BigInt {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        if self.limbs.len() > 1 {
            return Some(std::cmp::Ordering::Greater);
        }
        Some(self.limbs[0].cmp(other))
    }
}

impl BitAnd<u64> for BigInt {
    type Output = u64;

    fn bitand(self, other: u64) -> u64 {
        self.limbs[0] & other
    }
}

impl BitOrAssign<u64> for BigInt {
    fn bitor_assign(&mut self, other: u64) {
        self.limbs[0] |= other;
    }
}

impl BigInt {
    pub fn minus_one(&mut self) {
        // technically i--; but rust do not have decrement operator, thus a new function.
        let mut i = 0;
        while i < self.limbs.len() {
            if self.limbs[i] == 0 {
                self.limbs[i] = u64::MAX; // borrow from next limb
                i += 1;
            } else {
                self.limbs[i] -= 1;
                break;
            }
        }
    }
}
// Representation
impl BigInt {
    pub fn binary(&self) -> String {
        self.limbs
            .iter()
            .rev()
            .map(|l| format!("{:064b} ", l))
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod tests_constructors {
    use super::*;

    #[test]
    fn test_random() {
        let mut n: BigInt;

        for _i in 0..8 {
            n = BigInt::random(61);
            assert_eq!(n.limbs.len(), 1);
            assert!(n.limbs[0] < (1 << 61));
        }

        for _i in 0..8 {
            n = BigInt::random(64);
            assert_eq!(n.limbs.len(), 1);
        }

        for _i in 0..8 {
            n = BigInt::random(65);
            assert_eq!(n.limbs.len(), 2);
            assert!(n.limbs[1] < (1 << 1));
        }
    }

    #[test]
    fn test_from_binary() {
        let mut a = BigInt::from_binary("1000");
        assert_eq!(a.limbs.len(), 1);
        assert_eq!(a.limbs[0], 0x8);

        a = BigInt::from_binary(&("1000".to_owned() + &"1001".repeat(16) + &"0101".repeat(16)));
        assert_eq!(a.limbs.len(), 3);
        assert_eq!(a.limbs[0], 0x5555555555555555);
        assert_eq!(a.limbs[1], 0x9999999999999999);
        assert_eq!(a.limbs[2], 0x8);

        a = BigInt::from_binary(
            "11110000000100100011010001010110011110001001101010111100110111101111",
        );
        assert_eq!(a.limbs.len(), 2);
        assert_eq!(a.limbs[0], 0x123456789ABCDEF);
        assert_eq!(a.limbs[1], 0xF);
    }

    #[test]
    fn test_from_hex() {
        let mut a = BigInt::from_hex("123456789ABCDEF");
        assert_eq!(a.limbs.len(), 1);
        assert_eq!(a.limbs[0], 0x123456789ABCDEF);

        a = BigInt::from_hex("1233211234567234432456789A");
        assert_eq!(a.limbs.len(), 2);
        assert_eq!(a.limbs[0], 0x567234432456789A);
        assert_eq!(a.limbs[1], 0x1233211234);

        a = BigInt::from_hex("DEADBEEF0DEADBEEF1DEADBEEF2DEADBEEF3DEADBEEF4DEADBEEF5DEADBEEF6DEADBEEF7DEADBEEF8DEADBEEF9DEADBEEF0");
        assert_eq!(a.limbs.len(), 7);
        assert_eq!(a.limbs[0], 0xADBEEF9DEADBEEF0);
        assert_eq!(a.limbs[1], 0xBEEF7DEADBEEF8DE);
        assert_eq!(a.limbs[2], 0xEF5DEADBEEF6DEAD);
        assert_eq!(a.limbs[3], 0x3DEADBEEF4DEADBE);
        assert_eq!(a.limbs[4], 0xEADBEEF2DEADBEEF);
        assert_eq!(a.limbs[5], 0xDBEEF0DEADBEEF1D);
        assert_eq!(a.limbs[6], 0xDEA);
    }
}

mod tests_ops {
    use super::BigInt;

    #[test]
    fn test_add() {
        let a = BigInt::from_binary(&"1".repeat(777));
        let b = BigInt::from_binary(&"1".repeat(777));
        let c = BigInt::from_binary(&format!("{}0", "1".repeat(777)));
        assert!(a + b == c);
    }

    #[test]
    fn test_eq() {
        let a = BigInt::from_binary(&"1".repeat(10000));
        let b = BigInt::from_binary(&"1".repeat(10000));
        assert!(a == b);
    }

    #[test]
    fn test_bitor() {
        let mut a = BigInt::from_binary(&"10101010");
        a |= 1;
        let b = BigInt::from_binary(&"10101011");
        assert!(a == b);

        a = BigInt::from_binary(&"101".repeat(1000));
        a |= 1;
        let b = BigInt::from_binary(&"101".repeat(1000));
        assert!(a == b);
    }

    #[test]
    fn test_bitand_u64() {
        let mut a = BigInt::from_binary(&"1111");
        a &= 10u64; // 1010 in binary
        let b = BigInt::from_binary(&"1010");
        assert!(a == b);

        let mut a = BigInt::from_hex("FFFFFFFFFFFFFFFF");
        a &= 42u64;
        let b = BigInt::from_u64(42);
        assert!(a == b);

        let mut a = BigInt::from_binary(&"1".repeat(1000));
        a &= 1u64;
        let b = BigInt::from_u64(1);
        assert!(a == b);
    }
    #[test]
    fn test_partial_ord_u64() {
        let a = BigInt::from_u64(42);
        let b = 42u64;
        assert!(!(a < b));
        assert!(!(a > b));
        assert!(a <= b);
        assert!(a >= b);

        let a = BigInt::from_u64(100);
        let b = 42u64;
        assert!(!(a < b));
        assert!(a > b);
        assert!(!(a <= b));
        assert!(a >= b);

        let a = BigInt::from_u64(10);
        let b = 42u64;
        assert!(a < b);
        assert!(!(a > b));
        assert!(a <= b);
        assert!(!(a >= b));

        let a = BigInt::from_hex(&"ABCDEF1234567890");
        let b = 42u64;
        assert!(!(a < b));
        assert!(a > b);
        assert!(!(a <= b));
        assert!(a >= b);
    }
}
