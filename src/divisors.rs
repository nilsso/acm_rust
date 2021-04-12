use std::cmp::{Eq, PartialOrd};
use std::ops::{AddAssign, Div, DivAssign, Mul, Rem};

use num_traits::{One, Pow, Zero};

use crate::factor::factor;

/// Returns the integer divisors of an integer.
///
/// # Examples
/// ```
/// assert_eq!(acm::divisors::divisors(60), vec![1, 2, 4, 3, 6, 12, 5, 10, 20, 15, 30, 60]);
/// ```
pub fn divisors<T>(n: T) -> Vec<T>
where
    T: Zero + One + AddAssign + DivAssign + Eq + PartialOrd + Clone,
    for<'a> &'a T: Mul<T, Output = T> + Pow<usize, Output = T>,
    for<'b> T: DivAssign<&'b T>,
    for<'a, 'b> &'a T: Div<&'b T, Output = T> + Rem<&'b T, Output = T>,
{
    let mut res = vec![T::one()];
    for (factor, m) in factor(n).iter() {
        for d in res.clone() {
            for p in 1..m + 1 {
                res.push(&d * factor.pow(p));
            }
        }
    }
    res
}
