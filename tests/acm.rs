extern crate acm;

type ACM = acm::ArithmeticCongruenceMonoid<u32>;

fn helper(a: u32, b: u32, n: u32, ans: Vec<Vec<u32>>) {
    assert_eq!(ACM::new(a, b).unwrap().factor(n), &ans);
}

#[test]
fn acm_valid() {
    assert!(ACM::new(1, 4).is_ok());
    assert!(ACM::new(3, 6).is_ok());
    assert!(ACM::new(6, 15).is_ok());
}

#[test]
fn acm_invalid() {
    assert!(ACM::new(2, 4).is_err());
    assert!(ACM::new(3, 4).is_err());
    assert!(ACM::new(6, 16).is_err());
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

#[test]
fn acm_6_10_factor_1296() {
    helper(6, 10, 2_u32.pow(4) * 3_u32.pow(4), vec![vec![6, 6, 6, 6]])
}

