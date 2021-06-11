use num_traits::{One, Zero};
use std::mem::swap;
use std::ops::{Div, Mul, Sub};

//
pub fn ext_euclid<T>(a: T, b: T) -> [T; 3]
where
    T: Zero + One + PartialEq + Clone,
    for<'a> &'a T: Sub<T, Output = T>,
    for<'b> T: Mul<&'b T, Output = T>,
    for<'a, 'b> &'a T: Mul<&'b T, Output = T> + Div<&'b T, Output = T>,
{
    let mut prev = [a, T::one(), T::zero()];
    let mut curr = [b, T::zero(), T::one()];
    while curr[0] != T::zero() {
        let q = &prev[0] / &curr[0];
        for i in 0..=2 {
            swap(&mut prev[i], &mut curr[i]);
            curr[i] = &curr[i] - &q * &prev[i];
        }
    }
    prev
}

pub trait GCD {
    fn gcd(self, other: Self) -> Self;
}

impl<T> GCD for T
where
    T: Zero + One + PartialEq + Clone,
    for<'a> &'a T: Sub<T, Output = T>,
    for<'b> T: Mul<&'b T, Output = T>,
    for<'a, 'b> &'a T: Mul<&'b T, Output = T> + Div<&'b T, Output = T>,
{
    /// ```
    /// use acm::integers::GCD;
    ///
    /// let a: i32 = 23 * 8; // 184
    /// let b: i32 = 23 * 9; // 207
    /// assert_eq!(a.gcd(b), 23);
    /// assert_eq!(b.gcd(a), 23);
    /// ```
    fn gcd(self, other: Self) -> Self {
        ext_euclid(self, other)[0].clone()
    }
}

// TODO: Use a real primality test
pub fn is_prime(x: u32) -> bool {
    x > 1 && (2..x).all(|d| x % d != 0)
}

pub fn first_congruent_prime(mut a: u32, m: u32) -> Option<u32> {
    a %= m;
    if is_prime(a) {
        Some(a)
    } else if (a as i32).gcd(m as i32) == 1 {
        let p = (a..)
            .step_by(m as usize)
            .filter(|&x| is_prime(x))
            .next()
            .unwrap();
        Some(p)
    } else {
        None
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ModClass {
    a: u32,
    m: u32,
    first_prime: Option<u32>,
}

impl ModClass {
    pub fn new(a: u32, m: u32) -> Self {
        let first_prime = first_congruent_prime(a, m);
        Self { a, m, first_prime }
    }

    pub fn a(&self) -> u32 {
        self.a
    }

    pub fn m(&self) -> u32 {
        self.m
    }

    pub fn first_prime(&self) -> Option<u32> {
        self.first_prime
    }
}
