extern crate acm;

use acm::ArithmeticCongruenceMonoid as ACM;

fn helper(a: i32, b: i32, n: i32, ans: Vec<Vec<i32>>) {
    assert_eq!(ACM::<i32>::new(a, b).unwrap().factor(n), &ans);
}

#[test]
fn acm_valid() {
    assert!(ACM::<i32>::new(1, 4).is_ok());
    assert!(ACM::<i32>::new(3, 6).is_ok());
    assert!(ACM::<i32>::new(6, 15).is_ok());
}

#[test]
fn acm_invalid() {
    assert!(ACM::<i32>::new(2, 4).is_err());
    assert!(ACM::<i32>::new(3, 4).is_err());
    assert!(ACM::<i32>::new(6, 16).is_err());
}

#[test]
fn acm_1_4_factor_1() {
    helper(1, 4, 1, vec![vec![]])
}

#[test]
fn acm_1_4_factor_5() {
    helper(1, 4, 5, vec![vec![5]])
}

#[test]
fn acm_1_4_factor_25() {
    helper(1, 4, 25, vec![vec![5, 5]])
}

#[test]
fn acm_1_4_factor_125() {
    helper(1, 4, 125, vec![vec![5, 5, 5]])
}
