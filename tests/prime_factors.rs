extern crate acm_rust;
use acm_rust::prime_factors::prime_factors;

#[test]
fn prime_factors_of_2() {
    assert_eq!(prime_factors(2), vec![(2, 1)]);
}

#[test]
fn prime_factors_of_4() {
    assert_eq!(prime_factors(4), vec![(2, 2)]);
}

#[test]
fn prime_factors_of_5() {
    assert_eq!(prime_factors(5), vec![(5, 1)]);
}

#[test]
fn prime_factors_of_12() {
    assert_eq!(prime_factors(12), vec![(2, 2), (3, 1)]);
}

#[test]
fn prime_factors_of_60() {
    assert_eq!(prime_factors(60), vec![(2, 2), (3, 1), (5, 1)]);
}

#[test]
fn prime_factors_of_420() {
    assert_eq!(prime_factors(420), vec![(2, 2), (3, 1), (5, 1), (7, 1)]);
}
