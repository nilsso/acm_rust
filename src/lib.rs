pub mod prime_factors;
pub mod divisors;

pub struct ArithmeticCongruenceMonoid {
    a: u32,
    b: u32
}

impl ArithmeticCongruenceMonoid {
    pub fn new(a: u32, b: u32) -> ArithmeticCongruenceMonoid {
        if a % b != a.pow(2) % b {
            // TODO: Use `Result` instead of panic
            panic!("a and a^2 must be equivalent mod b");
        }
        ArithmeticCongruenceMonoid {
            a: a % b,
            b
        }
    }

    pub fn hilbert(b: u32) -> ArithmeticCongruenceMonoid {
        ArithmeticCongruenceMonoid::new(1, b)
    }

    pub fn a(&self) -> u32 { self.a }
    pub fn b(&self) -> u32 { self.b }

    pub fn factorize(n: u32) -> Vec<(u32,u32)>{
        prime_factors::prime_factors(n)
    }
}
