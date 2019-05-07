pub mod prime_factors;

pub struct ArithmeticCongruenceMonoid {
}

impl ArithmeticCongruenceMonoid {
    pub fn factorize(n: u32) -> Vec<(u32,u32)>{
        prime_factors::prime_factors(n)
    }
}
