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
    helper(1, 4, 5, vec![vec![5]])
}
