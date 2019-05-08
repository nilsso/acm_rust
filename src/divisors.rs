use std::collections::BTreeSet;
use common_macros::b_tree_set;

use crate::prime_factors::prime_factors;

pub fn divisors(n: u32) -> BTreeSet<u32> {
    let mut divisors = b_tree_set!{1};
    for (factor, m) in prime_factors(n) {
        for d in divisors.clone() {
            for p in 1..m+1 {
                divisors.insert(d*factor.pow(p));
            }
        }
    }
    divisors
}
