extern crate acm_rust;
use acm_rust::factorize::factorize;

#[test]
fn factorize_of_2() {
    assert_eq!(factorize(2), vec![(2, 1)]);
}

#[test]
fn factorize_of_4() {
    assert_eq!(factorize(4), vec![(2, 2)]);
}

#[test]
fn factorize_of_5() {
    assert_eq!(factorize(5), vec![(5, 1)]);
}

#[test]
fn factorize_of_12() {
    assert_eq!(factorize(12), vec![(2, 2), (3, 1)]);
}

#[test]
fn factorize_of_60() {
    assert_eq!(factorize(60), vec![(2, 2), (3, 1), (5, 1)]);
}

#[test]
fn factorize_of_420() {
    assert_eq!(factorize(420), vec![(2, 2), (3, 1), (5, 1), (7, 1)]);
}
