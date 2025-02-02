extern crate rand;
use rand::Rng;
use std::ops::{Add, BitAnd, BitOrAssign, ShrAssign, Mul, Sub, Shl, Rem};
#[derive(Clone)]
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

    pub fn assert_valid(&self) {
        // Ensure that if BigInt has more than one limb, the most significant limb is non-zero.
        if self.limbs.len() > 1 {
            assert!(*self.limbs.last().unwrap() != 0, "Invalid BigInt: highest limb is 0 when multiple limbs exist {:?}", self.binary());
        }
    }

    pub fn compact(&mut self) {
        while self.limbs.len() > 1 && *self.limbs.last().unwrap() == 0 {
            self.limbs.pop();
        }
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> BigInt {
        let max_len = std::cmp::max(self.limbs.len(), other.limbs.len());
        let mut result = Vec::with_capacity(max_len + 1);
        let mut carry: u128 = 0;

        for i in 0..max_len {
            let a = if i < self.limbs.len() { self.limbs[i] as u128 } else { 0 };
            let b = if i < other.limbs.len() { other.limbs[i] as u128 } else { 0 };
            let sum = a + b + carry;
            result.push(sum as u64);
            carry = sum >> 64;
        }

        if carry != 0 {
            result.push(carry as u64);
        }

        BigInt { limbs: result }
    }
}

impl Mul for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> BigInt {
        let len_self = self.limbs.len();
        let len_rhs = rhs.limbs.len();

        // Early return if either multiplicand is zero.
        if (len_self == 1 && self.limbs[0] == 0) || (len_rhs == 1 && rhs.limbs[0] == 0) {
            return BigInt::from_u64(0);
        }

        // Preallocate enough space for the full product.
        let mut result = vec![0u64; len_self + len_rhs];

        for i in 0..len_self {
            let mut carry: u128 = 0;
            for j in 0..len_rhs {
                let idx = i + j;
                let prod = (self.limbs[i] as u128)
                    * (rhs.limbs[j] as u128)
                    + (result[idx] as u128)
                    + carry;
                result[idx] = prod as u64;
                carry = prod >> 64;
            }
            result[i + len_rhs] = carry as u64;
        }

        // Normalize: remove trailing zero limbs while keeping at least one limb.
        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        BigInt { limbs: result }
    }
}

impl<'a, 'b> Sub<&'b BigInt> for &'a BigInt {
    type Output = BigInt;
    fn sub(self, other: &BigInt) -> BigInt {
        assert!(self >= other, "Subtraction underflow: left operand is smaller than right operand");
        let mut borrow = 0u64;
        let mut result = Vec::with_capacity(self.limbs.len());
        for (i, &limb) in self.limbs.iter().enumerate() {
            let other_limb = other.limbs.get(i).copied().unwrap_or(0);
            let (diff, did_borrow) = limb.overflowing_sub(other_limb + borrow);
            result.push(diff);
            borrow = if did_borrow { 1 } else { 0 };
        }
        // Normalize: remove any trailing zero limbs while ensuring at least one limb remains.
        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }
        BigInt { limbs: result }
    }
}

impl Shl<usize> for &BigInt {
    type Output = BigInt;
    fn shl(self, shift: usize) -> BigInt {
        let limb_shift = shift / 64;
        let bit_shift = shift % 64;
        let mut result = vec![0u64; limb_shift];
        let mut carry = 0u64;
        for &limb in &self.limbs {
            let new_limb = (limb << bit_shift) | carry;
            carry = if bit_shift == 0 { 0 } else { limb >> (64 - bit_shift) };
            result.push(new_limb);
        }
        if carry != 0 {
            result.push(carry);
        }
        BigInt { limbs: result }
    }
}

impl BigInt {
    pub fn bit_length(&self) -> usize {
        // Assumes BigInt is normalized (i.e. no extra zero limbs at the end)
        let ms = *self.limbs.last().unwrap();
        let bits = 64 - ms.leading_zeros() as usize;
        (self.limbs.len() - 1) * 64 + bits
    }

    fn cmp_bigint(&mut self, other: &BigInt) -> std::cmp::Ordering {
        while self.limbs.len() > 1 && self.limbs[self.limbs.len() - 1] == 0 {
            self.limbs.pop();
        }
        use std::cmp::Ordering;
        if self.limbs.len() != other.limbs.len() {
            return self.limbs.len().cmp(&other.limbs.len());
        }
        for (&a, &b) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
            if a != b {
                return a.cmp(&b);
            }
        }
        Ordering::Equal
    }
}

impl Rem for &BigInt {
    type Output = BigInt;
    fn rem(self, rhs: &BigInt) -> BigInt {
        // Panic if trying to mod by 0.
        if rhs.limbs.len() == 1 && rhs.limbs[0] == 0 {
            panic!("Division by zero in modulus operation");
        }
        // If self is smaller than rhs, the remainder is self.
        let mut dividend = self.clone();
        if dividend.cmp_bigint(rhs) == std::cmp::Ordering::Less {
            return dividend.clone();
        }
        let shift = dividend.bit_length() - rhs.bit_length();
        for i in (0..=shift).rev() {
            let candidate = rhs << i;
            if dividend.cmp_bigint(&candidate) != std::cmp::Ordering::Less {
                dividend = &dividend - &candidate;
            }
        }
        dividend.clone()
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

impl Eq for BigInt {}


impl PartialOrd<u64> for BigInt {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        let mut s = self.clone();
        // while s.limbs.len() > 1 && s.limbs[s.limbs.len() - 1] == 0 {
        //     println!("{} {}", s.binary(), other);
        //     s.limbs.pop();
        // }
        // println!("{} {}", s.binary(), other);

        if s.limbs.len() > 1 {
            return Some(std::cmp::Ordering::Greater);
        }
        Some(s.limbs[0].cmp(other))
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Compare based on the number of limbs first.
        if self.limbs.len() != other.limbs.len() {
            return Some(self.limbs.len().cmp(&other.limbs.len()));
        }
        // If both have the same length, compare from the most significant limb.
        for (a, b) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
            if a < b {
                return Some(std::cmp::Ordering::Less);
            } else if a > b {
                return Some(std::cmp::Ordering::Greater);
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl std::cmp::Ord for BigInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Since our BigInt is always non-negative and normalized,
        // we can safely unwrap the result of partial_cmp.
        self.partial_cmp(other).unwrap()
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
        let total_limbs = self.limbs.len();
        let limb_offset = shift / 64;
        let bit_offset = shift % 64;

        // If the shift moves beyond the available limbs, zero out the number.
        if limb_offset >= total_limbs {
            self.limbs.clear();
            self.limbs.push(0);
            return;
        }

        let new_len = total_limbs - limb_offset;

        // Shift the limbs in place.
        for i in 0..new_len {
            // Lower part comes from the current limb shifted right.
            let lower = self.limbs[i + limb_offset] >> bit_offset;
            // Upper part comes from the next limb (if available) shifted left.
            let upper = if bit_offset > 0 && i + limb_offset + 1 < total_limbs {
                self.limbs[i + limb_offset + 1] << (64 - bit_offset)
            } else {
                0
            };
            self.limbs[i] = lower | upper;
        }
        self.limbs.truncate(new_len);

        // Normalize: Remove trailing zero limbs (if the most significant limb is zero)
        while self.limbs.len() > 1 && *self.limbs.last().unwrap() == 0 {
            // println!("actually pop");
            self.limbs.pop();
        }
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

    pub fn modpow(&self, exp: &BigInt, modulus: &BigInt) -> BigInt {
        self.assert_valid();
        exp.assert_valid();
        modulus.assert_valid();
        // println!("modulus {}", modulus.binary());
        // println!("exp {}", exp.binary());
        // println!("self {}", self.binary());
        let mut result = BigInt::from_u64(1);
        let mut base = self % modulus;
        let mut e = exp.clone();
        while e > 0 {
            // println!("begin {}", e.binary());
            let start = std::time::Instant::now();
            if !e.is_even() {
                result = &(&result * &base) % modulus;
            }
            // println!("Time taken for this 1 operation: {:?}", start.elapsed());

            let shift_start = std::time::Instant::now();
            e >>= 1;
            // println!("Time taken for shift 2 operation: {:?}", shift_start.elapsed());

            // println!("e {}", e.binary());

            let op_start = std::time::Instant::now();
            base = &(&base * &base) % modulus;
            // println!("base {}", base.binary());
            // println!("modulus {}", modulus.binary());
            // println!("num limbs of base: {}", base.bit_length());
            // println!("Last limb of base: {}", base.limbs.last().unwrap());
            // println!("Time taken for squaring operation: {:?}", op_start.elapsed());
        }
        result
    }

    pub fn modpow_u32(&self, exp: u32, modulus: &BigInt) -> BigInt {
        self.modpow(&BigInt::from_u64(exp as u64), modulus)
    }
}
// Representation
impl BigInt {
    pub fn binary(&self) -> String {
        self.limbs
            .iter()
            .rev()
            .map(|l| format!("{:064b}", l))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn hex(&self) -> String {
        self.limbs
            .iter()
            .rev()
            .map(|l| format!("{:016x}", l))
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
        assert!(&a + &b == c);
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
    fn test_overloaded_ops() {
        // Test addition
        let a = BigInt::from_u64(123);
        let b = BigInt::from_u64(456);
        let sum = &a + &b;
        assert_eq!(sum, BigInt::from_u64(579));

        // Test subtraction
        let diff = &b - &a;
        assert_eq!(diff, BigInt::from_u64(333));

        // Test multiplication
        let x = BigInt::from_u64(12);
        let y = BigInt::from_u64(34);
        let product = &x * &y;
        assert_eq!(product, BigInt::from_u64(408));

        // Test left shift (<<)
        let one = BigInt::from_u64(1);
        let shifted = &one << 5;
        assert_eq!(shifted, BigInt::from_u64(32));

        // Test remainder (%)
        let ten = BigInt::from_u64(10);
        let three = BigInt::from_u64(3);
        let rem = &ten % &three;
        assert_eq!(rem, BigInt::from_u64(1));

        // Test bitwise or assignment (|=)
        let mut c = BigInt::from_u64(8); // binary 1000
        c |= 3; // 3 is 0011 in binary, so 1000 | 0011 should be 1011 (which is 11 in decimal)
        assert_eq!(c, BigInt::from_u64(11));
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

    #[test]
    fn test_shrassign() {
        // Test basic right shift assignment.
        let mut a = BigInt::from_u64(1024);
        a >>= 5; // 1024 >> 5 == 32
        assert_eq!(a, BigInt::from_u64(32));

        // Test shifting by zero leaves the number unchanged.
        let mut b = BigInt::from_u64(12345);
        b >>= 0;
        assert_eq!(b, BigInt::from_u64(12345));

        // Test shifting a small number past its bit-width, which should yield zero.
        let mut c = BigInt::from_u64(10);
        c >>= 4; // 10 >> 4 yields 0 (10/16 == 0)
        assert_eq!(c, BigInt::from_u64(0));

        // Multi-limb test:
        // Construct a BigInt representing 2^64 by using a binary string: "1" followed by 64 zeros.
        let binary_str = format!("1{}", "0".repeat(64));
        let mut d = BigInt::from_binary(&binary_str); // This yields a two-limb number: limbs[0] == 0, limbs[1] == 1.
        d >>= 1; // Right shifting 2^64 by 1 should result in 2^63.
        assert_eq!(d, BigInt::from_u64(0x8000000000000000));
    }

 // Start Generation Here
 #[test]
 fn test_rem() {
     // Test modulo with dividend greater than divisor.
     let a = BigInt::from_u64(100);
     let b = BigInt::from_u64(30);
     let r = &a % &b;
     assert_eq!(r, BigInt::from_u64(10));

     // Test modulo when dividend is smaller than divisor (the remainder should be the dividend itself).
     let a = BigInt::from_u64(42);
     let b = BigInt::from_u64(100);
     let r = &a % &b;
     assert_eq!(r, a);

     // Test modulo where the dividend is exactly divisible by the divisor (remainder is zero).
     let a = BigInt::from_u64(12345);
     let b = BigInt::from_u64(5);
     let r = &a % &b;
     assert_eq!(r, BigInt::from_u64(0));

     // Multi-limb test:
     // Construct a BigInt representing 2^64 by using a binary string: "1" followed by 64 zeros.
     // 2^64 = 18446744073709551616. Since 18446744073709551616 mod 100 is 16, we expect the remainder to be 16.
     let a = BigInt::from_binary(&format!("1{}", "0".repeat(64)));
     let r = &a % &BigInt::from_u64(100);
     assert_eq!(r, BigInt::from_u64(16));

     let a: BigInt = BigInt::random(65536);
     let b: BigInt = BigInt::random(99);
     let mut base = &a % &b;
    //  println!("b {}", b.bit_length());
     for _ in 0..10 {
        // println!("before {}", base.bit_length());
        base = &(&base * &base) % &b;
        // println!("after {}", base.bit_length());
        assert!(base.bit_length() <= b.bit_length());
     }
 }

#[test]
fn test_modpow() {
    // // Test case 1: Exponent zero. Any base to the power 0 should return 1 mod modulus.
    // let base = BigInt::from_u64(123456789);
    // let exp = BigInt::from_u64(0);
    // let modulus = BigInt::from_u64(98765);
    // let result = base.modpow(&exp, &modulus);
    // assert_eq!(result, BigInt::from_u64(1));

    // // Test case 2: Small exponent.
    // // 2^10 = 1024, and 1024 mod 1000 equals 24.
    // let base = BigInt::from_u64(2);
    // let exp = BigInt::from_u64(10);
    // let modulus = BigInt::from_u64(1000);
    // let result = base.modpow(&exp, &modulus);
    // assert_eq!(result, BigInt::from_u64(24));

    // // Test case 3: Another simple example.
    // // 3^5 = 243, and 243 mod 13 equals 9.
    // let base = BigInt::from_u64(3);
    // let exp = BigInt::from_u64(5);
    // let modulus = BigInt::from_u64(13);
    // let result = base.modpow(&exp, &modulus);
    // assert_eq!(result, BigInt::from_u64(9));

    // Test case 4: Multi-limb computation.
    // 2^20 = 1048576, and 1048576 mod 17 equals 16.
    let base = BigInt::random(177);
    let exp = BigInt::random(123);
    let modulus = BigInt::random(99);

    let modulus = BigInt::from_binary("00000000000000000000000000000000000000000000000000000000000000000111001100101010110101011100011100111100101110001111111100110011");
    // let exp = BigInt::from_binary("0011100110010101011010101110001110011110010111000111111110011001");

    let result = base.modpow(&exp, &modulus);
    // println!("result {}", result.binary());

    assert!(result.bit_length() <= modulus.bit_length());
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
