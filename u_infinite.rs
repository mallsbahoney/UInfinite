// uInfinite logic
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct UInfinite {
    digits: Vec<u8>, // Least-significant digit first
}

impl UInfinite {
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return Err("Invalid digit in input");
        }
        let digits = s.chars().rev().map(|c| c as u8 - b'0').collect();
        Ok(Self { digits })
    }

    pub fn to_string(&self) -> String {
        if self.digits.is_empty() {
            return "0".to_string();
        }
        self.digits.iter().rev().map(|d| (d + b'0') as char).collect()
    }

    pub fn mul(&self, other: &Self) -> Result<Self, &'static str> {
        let mut result = vec![0u8; self.digits.len() + other.digits.len()];
        for (i, &a) in self.digits.iter().enumerate() {
            for (j, &b) in other.digits.iter().enumerate() {
                result[i + j] += a * b;
                let mut k = i + j;
                while result[k] >= 10 {
                    let carry = result[k] / 10;
                    result[k] %= 10;
                    k += 1;
                    if k == result.len() {
                        result.push(0);
                    }
                    result[k] += carry;
                }
            }
        }
        while result.last() == Some(&0) && result.len() > 1 {
            result.pop();
        }
        Ok(Self { digits: result })
    }
}

    pub fn add(&self, other: &Self) -> Result<Self, &'static str> {
        let max_len = self.digits.len().max(other.digits.len());
        let mut result = Vec::with_capacity(max_len + 1);
        let mut carry = 0;

        for i in 0..max_len {
            let a = *self.digits.get(i).unwrap_or(&0);
            let b = *other.digits.get(i).unwrap_or(&0);
            let sum = a + b + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        if carry > 0 {
            result.push(carry);
        }

        Ok(Self { digits: result })
    }

    pub fn sub(&self, other: &Self) -> Result<Self, &'static str> {
        if self < other {
            return Err("Subtraction underflow");
        }

        let mut result = Vec::with_capacity(self.digits.len());
        let mut borrow = 0;

        for i in 0..self.digits.len() {
            let a = self.digits[i];
            let b = *other.digits.get(i).unwrap_or(&0) + borrow;
            if a >= b {
                result.push(a - b);
                borrow = 0;
            } else {
                result.push(a + 10 - b);
                borrow = 1;
            }
        }

        while result.last() == Some(&0) && result.len() > 1 {
            result.pop();
        }

        Ok(Self { digits: result })
    }

    pub fn div(&self, other: &Self) -> Result<Self, &'static str> {
        if other.digits.is_empty() || other.to_string() == "0" {
            return Err("Division by zero");
        }

        let mut remainder = UInfinite { digits: vec![] };
        let mut quotient = UInfinite { digits: vec![] };

        for &digit in self.digits.iter().rev() {
            remainder.digits.insert(0, digit);
            let mut count = 0;
            while remainder >= *other {
                remainder = remainder.sub(other)?;
                count += 1;
            }
            quotient.digits.push(count);
        }

        quotient.digits.reverse();
        while quotient.digits.last() == Some(&0) && quotient.digits.len() > 1 {
            quotient.digits.pop();
        }

        Ok(quotient)
    }


impl UInfinite {
    pub fn from_char_string(s: &str) -> Self {
        let chars = s.chars().collect::<Vec<char>>();
        let bytes = chars.into_iter().map(|c| c as u8).collect();
        Self { digits: bytes }
    }

    pub fn to_char_string(&self) -> String {
        self.digits.iter().map(|b| *b as char).collect()
    }

    pub fn is_char_mode(&self) -> bool {
        self.digits.iter().any(|&b| b < b'0' || b > b'9')
    }
}


pub trait MathMode {
    fn from_digit_string(s: &str) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn to_digit_string(&self) -> String;
}

pub trait CharMode {
    fn from_char_string(s: &str) -> Self
    where
        Self: Sized;
    fn to_char_string(&self) -> String;
    fn is_char_mode(&self) -> bool;
}

impl MathMode for UInfinite {
    fn from_digit_string(s: &str) -> Result<Self, &'static str> {
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return Err("Invalid digit in input");
        }
        let digits = s.chars().rev().map(|c| c as u8 - b'0').collect();
        Ok(Self { digits })
    }

    fn to_digit_string(&self) -> String {
        if self.digits.is_empty() {
            return "0".to_string();
        }
        self.digits.iter().rev().map(|d| (d + b'0') as char).collect()
    }
}

impl CharMode for UInfinite {
    fn from_char_string(s: &str) -> Self {
        let chars = s.chars().collect::<Vec<char>>();
        let bytes = chars.into_iter().map(|c| c as u8).collect();
        Self { digits: bytes }
    }

    fn to_char_string(&self) -> String {
        self.digits.iter().map(|b| *b as char).collect()
    }

    fn is_char_mode(&self) -> bool {
        self.digits.iter().any(|&b| b < b'0' || b > b'9')
    }
}

impl UInfinite {
    /// Automatically chooses how to interpret based on content
    pub fn auto_from(s: &str) -> Result<Self, &'static str> {
        if s.chars().all(|c| c.is_ascii_digit()) {
            <Self as MathMode>::from_digit_string(s)
        } else {
            Ok(<Self as CharMode>::from_char_string(s))
        }
    }

    /// Outputs as math-safe string or raw char string depending on mode
    pub fn auto_to_string(&self) -> String {
        if self.is_char_mode() {
            <Self as CharMode>::to_char_string(self)
        } else {
            <Self as MathMode>::to_digit_string(self)
        }
    }
}
