extern crate acm_rust;
use acm_rust::ArithmeticCongruenceMonoid;

fn helper(a: u32, b: u32, n: u32, ans: Vec<Vec<u32>>) {
    assert_eq!(
        ArithmeticCongruenceMonoid::new(a, b).factorize(n).to_vec(),
        ans
    );
}

#[test]
fn acm_1_4_factorize_1() {
    helper(1, 4, 1, vec![vec![]])
}

#[test]
fn acm_1_4_factorize_5() {
    helper(1, 4, 5, vec![vec![5]])
}

#[test]
fn acm_1_4_factorize_25() {
    helper(1, 4, 25, vec![vec![5, 5]])
}

#[test]
fn acm_1_4_factorize_125() {
    helper(1, 4, 125, vec![vec![5, 5, 5]])
}

#[test]
fn acm_1_4_factorize_151253545() {
    helper(1, 4, 151253545, vec![vec![5, 593, 51013]])
}
