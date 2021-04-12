extern crate acm;

use acm::factor::factor;

fn helper(n: u64, ans: Vec<(u64, usize)>) {
    let mut fs = factor(n);
    fs.sort();
    assert_eq!(fs, ans);
}

#[test]
fn factor_of_2() {
    helper(2, vec![(2, 1)]);
}

#[test]
fn factor_of_4() {
    helper(4, vec![(2, 2)]);
}

#[test]
fn factor_of_5() {
    helper(5, vec![(5, 1)]);
}

#[test]
fn factor_of_12() {
    helper(12, vec![(2, 2), (3, 1)]);
}

#[test]
fn factor_of_60() {
    helper(60, vec![(2, 2), (3, 1), (5, 1)]);
}

#[test]
fn factor_of_420() {
    helper(420, vec![(2, 2), (3, 1), (5, 1), (7, 1)]);
}
