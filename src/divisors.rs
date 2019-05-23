use crate::factorize;

/// Returns the integer divisors of an integer.
pub fn divisors(n: u32) -> Vec<u32> {
    let mut divisors = vec![1];
    for (factor, m) in factorize(n) {
        for d in divisors.clone() {
            for p in 1..m + 1 {
                divisors.push(d * factor.pow(p));
            }
        }
    }
    divisors
}
