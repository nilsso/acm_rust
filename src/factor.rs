use std::cmp::{Eq, PartialOrd};
use std::ops::{AddAssign, Div, DivAssign, Rem};

use num_traits::{One, Pow, Zero};

/// Returns the prime power factorization of an integer.
///
/// # Examples
/// ```
/// assert_eq!(acm::factor::factor(120), vec![(2, 3), (3, 1), (5, 1)]);
/// ```
pub fn factor<T>(mut n: T) -> Vec<(T, usize)>
where
    T: Zero + One + AddAssign + DivAssign + Eq + PartialOrd + Clone,
    for<'a> &'a T: Pow<usize, Output = T>,
    for<'b> T: DivAssign<&'b T>,
    for<'a, 'b> &'a T: Div<&'b T, Output = T> + Rem<&'b T, Output = T>,
{
    // Note:
    // Moving to generic element types with support for extensible integers (e.g. BigInt and Int
    // from num_bigint and ramp respectively) requires a lot of care in what operations are being
    // used. In particular, these extensible integer types only have Clone and not Copy, so
    // iterative algorithms must operate using references. For this, higher order trait bounds (the
    // `for<'a, ...>` lines) were then necessary.

    // Prime factors (and their powers)
    let mut pfs: Vec<(T, usize)> = Vec::new();
    // Divisor
    let mut d = T::one() + T::one();
    while &n > &T::one() {
        while (&n % &d) != T::zero() {
            d += T::one();
        }
        let mut q = &n / &d;
        let mut i = 1_usize;
        while &q % &d == T::zero() {
            q /= &d;
            i += 1;
        }
        pfs.push((d.clone(), i));
        n /= (&d).pow(i);
    }
    pfs
}
