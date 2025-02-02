extern crate rand;
use rand::Rng;
use std::ops::Add;
use std::ops::BitAnd;
use std::ops::BitOrAssign;
use std::ops::ShrAssign;

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

impl ShrAssign<u32> for BigInt {
    fn shr_assign(&mut self, rhs: u32) {
        let shift = rhs as usize;
        let num_limbs = self.limbs.len();
        let limb_shift = shift / 64;
        let bit_shift = shift % 64;
        if limb_shift >= num_limbs {
            self.limbs = vec![0];
            return;
        }
        let new_len = num_limbs - limb_shift;
        let mut new_limbs = Vec::with_capacity(new_len);
        for i in 0..new_len {
            let current = self.limbs[i + limb_shift];
            let next = if i + limb_shift + 1 < num_limbs {
                self.limbs[i + limb_shift + 1]
            } else {
                0
            };
            if bit_shift == 0 {
                new_limbs.push(current);
            } else {
                new_limbs.push((current >> bit_shift) | (next << (64 - bit_shift)));
            }
        }
        self.limbs = new_limbs;
    }
}

impl BitOrAssign<u64> for BigInt {
    fn bitor_assign(&mut self, other: u64) {
        self.limbs[0] |= other;
    }
}

// a bunch of helper arithmetic functions
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

    pub fn is_even(&self) -> bool {
        self.limbs[0] % 2 == 0
    }

    pub fn trailing_zeros(&self) -> u32 {
        // Check each limb from least significant to most significant
        for (i, limb) in self.limbs.iter().enumerate() {
            if *limb != 0 {
                // Found a non-zero limb, count its trailing zeros
                return (i as u32) * 64 + limb.trailing_zeros();
            }
        }
        // All limbs are zero
        (self.limbs.len() as u32) * 64
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
impl std::fmt::Debug for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BigInt({})", self.binary())
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
        let a_u64 = a & 10u64; // 1010 in binary
        let b = BigInt::from_binary(&"1010");
        assert!(b == a_u64);

        let mut a = BigInt::from_hex("FFFFFFFFFFFFFFFF");
        let a_u64 = a & 42u64;
        let b = BigInt::from_u64(42);
        assert!(b == a_u64);

        let mut a = BigInt::from_binary(&"1".repeat(1000));
        let a_u64 = a & 1u64;
        let b = BigInt::from_u64(1);
        assert!(b == a_u64);
    }

    #[test]
    fn test_shr_assign() {
        // Test right-shifting a single-limb BigInt by less than 64 bits.
        // For example, 11 (binary 1011) >> 1 should be 5 (binary 101).
        let mut a = BigInt::from_u64(11);
        a >>= 1;
        assert_eq!(a, BigInt::from_u64(5));

        // Test right-shifting a single-limb BigInt by 64 bits yields 0.
        let mut b = BigInt::from_u64(12345);
        b >>= 64;
        assert_eq!(b, BigInt::from_u64(0));

        // Test right-shifting a multi-limb BigInt by a non-multiple of 64 bits.
        // Create a multi-limb BigInt representing 2^70 - 1 using a binary string of 70 ones.
        let original = BigInt::from_binary(&"1".repeat(70));
        let mut c = original;
        c >>= 3; // (2^70 - 1) >> 3 should equal 2^67 - 1, which is represented by 67 ones in binary.
        let expected = BigInt::from_binary(&"1".repeat(67));
        assert_eq!(c, expected);

        // Test right-shifting a multi-limb BigInt by exactly 64 bits.
        // (2^70 - 1) >> 64 equals floor((2^70 - 1)/2^64) which is 63.
        let original = BigInt::from_binary(&"1".repeat(70));
        let mut d = original;
        d >>= 64;
        assert_eq!(d, BigInt::from_u64(63));
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


mod tests_arithmetic {
    use super::*;

    #[test]
    fn test_minus_one() {
        // Test simple subtraction without borrowing.
        let mut a = BigInt::from_u64(10);
        a.minus_one();
        assert_eq!(a, BigInt::from_u64(9));

        // Test edge case: subtracting one from one.
        let mut b = BigInt::from_u64(1);
        b.minus_one();
        assert_eq!(b, BigInt::from_u64(0));

        // Test subtraction that requires borrowing across limbs.
        // Construct a two-limb BigInt representing 2^64.
        let bin_str = format!("1{}", "0".repeat(64));
        let mut c = BigInt::from_binary(&bin_str);
        c.minus_one();
        // After subtraction, the internal limbs should be [u64::MAX, 0]
        let expected = BigInt { limbs: vec![u64::MAX, 0] };
        assert_eq!(c, expected);
    }

    #[test]
    fn test_trailing_zeros() {
        let mut a = BigInt::from_binary("1");
        assert_eq!(a.trailing_zeros(), 0);

        a = BigInt::from_binary("100");
        assert_eq!(a.trailing_zeros(), 2);

        a = BigInt::from_binary("1000000");
        assert_eq!(a.trailing_zeros(), 6);

        // Test across limb boundary
        a = BigInt::from_binary(&("1".repeat(64) + &"0".repeat(5)));
        assert_eq!(a.trailing_zeros(), 5);

        // Test multiple limbs of zeros
        a = BigInt::from_binary(&("1".repeat(64) + &"0".repeat(128)));
        assert_eq!(a.trailing_zeros(), 128);
        a = BigInt::from_binary(&("1".repeat(64) + &"0".repeat(147)));
        assert_eq!(a.trailing_zeros(), 147);

        // Test all zeros
        a = BigInt::from_binary(&"0".repeat(256));
        assert_eq!(a.trailing_zeros(), 256);
    }
}
