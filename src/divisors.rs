use crate::factorize;

/// Returns the integer divisors of an integer.
///
/// # Examples
/// ```
/// assert_eq!(acm::divisors(60), [1, 2, 4, 3, 6, 12, 5, 10, 20, 15, 30, 60]);
/// ```
pub fn divisors(n: u64) -> Vec<u64> {
    let mut res = vec![1];
    for (factor, m) in factorize(n) {
        for d in res.clone() {
            for p in 1..m + 1 {
                res.push(d * factor.pow(p as u32));
            }
        }
    }
    res
}
